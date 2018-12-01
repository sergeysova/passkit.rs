extern crate crypto;
extern crate fs_extra;
extern crate keychain_services;
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
    CantSerializePass,
    CantWritePassFile(String),
    CantCalculateHashes,
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
            CantSerializePass => "Can't serialize pass.json".to_string(),
            CantWritePassFile(cause) => format!("Can't write pass.json {}", cause),
            CantCalculateHashes => "Can't calculate hashes for temp directory".to_string(),
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
}

impl PassSource {
    pub fn new<S: Into<String>>(source: S) -> PassSource {
        PassSource {
            source_directory: source.into(),
            ..Default::default()
        }
    }

    /// Add exists pass to source
    pub fn add_pass(&mut self, pass: Pass) -> &mut Self {
        self.pass_content = Some(pass);
        self
    }

    /// Create .pkpass file in target directory
    pub fn build_pkpass(&mut self) -> PassResult<()> {
        self.resolve_pass_content()?;
        let tmp = Self::create_tmp_dir()?;

        self.copy_source_files_to(tmp.path())?;
        self.write_pass_file_to(tmp.path())?;
        self.calculate_hashes_of(tmp.path())?;
        Ok(())
    }

    /// Parse pass.json from source directory if Pass not provided
    fn resolve_pass_content(&mut self) -> PassResult<()> {
        if self.pass_content.is_none() && self.is_pass_file_exists_in_source() {
            self.pass_content = Some(self.read_pass_file_from_source()?);
        }
        Ok(())
    }

    fn is_pass_file_exists_in_source(&self) -> bool {
        self.pass_source_file_path().exists()
    }

    fn read_pass_file_from_source(&self) -> PassResult<Pass> {
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

    fn create_tmp_dir() -> PassResult<TempDir> {
        TempDir::new("passsource").map_err(|_| PassCreateError::CantCreateTempDir)
    }

    fn write_pass_file_to(&self, dir: &path::Path) -> PassResult<()> {
        if !self.is_pass_file_exists_in_source() {
            if let Some(pass) = &self.pass_content {
                let serialized = serde_json::to_string_pretty(&pass)
                    .map_err(|_| PassCreateError::CantSerializePass)?;

                let pass_file_path = dir.join("pass.json");
                fs::write(pass_file_path, serialized)
                    .map_err(|err| PassCreateError::CantWritePassFile(err.to_string()))?;
            }
        }
        Ok(())
    }

    fn copy_source_files_to(&mut self, dir: &path::Path) -> PassResult<()> {
        fn walk(from: &path::Path, to: &path::Path) -> std::io::Result<()> {
            for entry in fs::read_dir(&from)? {
                // println!("{:?}", entry?);
                let entry = entry?;
                let entry_path = &entry.path();
                let target = entry_path
                    .strip_prefix(&from)
                    .map_err(|__| std::io::Error::from(std::io::ErrorKind::Other))?;
                fs::copy(entry.path(), to.join(target))?;
            }

            Ok(())
        }

        walk(&path::Path::new(&self.source_directory), dir)
            .map_err(|_| PassCreateError::CantCopySourceToTemp)?;

        Ok(())
    }

    fn calculate_hashes_of(&mut self, dir: &path::Path) -> PassResult<()> {
        fn enumerate(dir: &path::Path) -> std::io::Result<Manifest> {
            let mut manifest = Manifest::new();

            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                if entry.metadata()?.is_file() {
                    let file_name = format!("{:?}", entry.file_name());
                    let content = read_file_to_vec(entry.path())?;
                    let hash = get_hash(&content);

                    manifest.insert(file_name, hash);
                }
            }

            Ok(manifest)
        }

        self.manifest = enumerate(&dir).map_err(|_| PassCreateError::CantCalculateHashes)?;
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
