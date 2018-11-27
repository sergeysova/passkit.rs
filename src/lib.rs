extern crate crypto;
extern crate fs_extra;
extern crate openssl;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tempdir;
extern crate zip;

mod field;
mod pass;
mod personalization;
mod util;

use crypto::{digest::Digest, sha1::Sha1};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::prelude::*;
use std::path;
use tempdir::TempDir;

pub use field::*;
pub use pass::*;
pub use personalization::*;

// use Failure
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PassCreateError {
    CantReadTempDir,
    CantReadEntry(String),
    CantParsePassFile(String),
    PassContentNotFound,
    CantCreateTempDir,
    CantCopySourceToTemp,
}

impl fmt::Display for PassCreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PassCreateError::*;
        let stringified = match self {
            CantReadTempDir => "Can't read temporary directory".to_string(),
            CantReadEntry(cause) => format!("Can't read {}", cause),
            CantParsePassFile(cause) => format!("pass.json invalid: {}", cause),
            PassContentNotFound => {
                "Please, provide pass.json or instance of Pass with add_pass() method".to_string()
            }
            CantCreateTempDir => "Can't create temporary directory. Check rights".to_string(),
            CantCopySourceToTemp => "Can't copy source files to temp directory".to_string(),
        };
        write!(f, "PassCreateError: {}", stringified)
    }
}

impl std::error::Error for PassCreateError {}

type PassResult<T> = Result<T, PassCreateError>;
type Manifest = HashMap<String, String>;

/// Describes .pass directory with source files
#[derive(Debug, Default)]
pub struct PassSource {
    /// place where images contains
    source_directory: String,

    /// hashes of files to generate manifest.json
    manifest: Manifest,

    /// content of the pass
    pass_content: Option<Pass>,

    /// place of temporary source of zip archive
    temp_dir: Option<TempDir>,
}

impl PassSource {
    pub fn new<S: Into<String>>(source: S) -> PassSource {
        PassSource {
            source_directory: source.into(),
            ..Default::default()
        }
    }

    pub fn add_pass(&mut self, pass: Pass) -> &mut Self {
        self.pass_content = Some(pass);
        self
    }

    /// Create .pkpass file in target directory
    pub fn build_pkpass(&mut self) -> PassResult<()> {
        self.resolve_pass_content()?;
        self.copy_source_files_to_temp_dir()?;
        self.calculate_hashes_for_manifest()?;
        Ok(())
    }

    /// Parse pass.json from source directory if Pass not provided
    fn resolve_pass_content(&mut self) -> PassResult<()> {
        if self.pass_content.is_none() {
            if self.is_pass_file_exists_in_source() {
                self.pass_content = Some(self.read_pass_source_file()?);
            }
        }
        Ok(())
    }

    fn is_pass_file_exists_in_source(&self) -> bool {
        self.pass_source_file_path().exists()
    }

    fn read_pass_source_file(&self) -> PassResult<Pass> {
        let content = read_file_to_vec(self.pass_source_file_path())
            .map_err(|_| PassCreateError::CantReadEntry("pass.json".to_string()))?;
        let pass: Pass = serde_json::from_slice(&content)
            .map_err(|cause| PassCreateError::CantParsePassFile(cause.to_string()))?;
        Ok(pass)
    }

    fn pass_source_file_path(&self) -> Box<path::Path> {
        let path = path::Path::new(&self.source_directory).join("pass.json");
        path.into_boxed_path()
    }

    fn copy_source_files_to_temp_dir(&mut self) -> PassResult<()> {
        use fs_extra::dir::{copy, CopyOptions};

        self.create_temp_dir()?;

        if let Some(ref tmp_dir) = self.temp_dir {
            copy(&self.source_directory, tmp_dir, &CopyOptions::new())
                .map_err(|_| PassCreateError::CantCopySourceToTemp)?;
        }

        Ok(())
    }

    fn create_temp_dir(&mut self) -> PassResult<()> {
        let tmp = TempDir::new("passsource").map_err(|_| PassCreateError::CantCreateTempDir)?;
        self.temp_dir = Some(tmp);
        Ok(())
    }

    fn temp_dir_path(&self) -> &path::Path {
        let dir = self.temp_dir.unwrap();
        dir.path()
    }

    fn calculate_hashes_for_manifest(&self) -> PassResult<()> {
        let mut manifest = Manifest::new();
        let dir = self.temp_dir.ok_or(PassCreateError::CantReadTempDir)?.path();
        let list = fs::read_dir(&dir).map_err(|_| PassCreateError::CantReadTempDir)?;

        for entry in list {
            let entry = entry.map_err(|err| PassCreateError::CantReadEntry(err.to_string()))?;
            let content = read_file_to_vec(entry.path())
                .map_err(|err| PassCreateError::CantReadEntry(err.to_string()))?;
            let hash = get_hash(&content);
            let file_name = format!("{:?}", entry.file_name());

            println!("— {} >> {}", file_name, hash);
            manifest.insert(file_name, hash);
        }

        Ok(())
    }
}

fn read_file_to_vec<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut file = fs::File::open(path.as_ref())?;
    let length = file.metadata()?.len();
    let buffer = {
        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        file.read_to_end(&mut buffer)?;
        buffer
    };

    Ok(buffer)
}

#[inline]
fn get_hash(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.input(content);
    hasher.result_str()
}
