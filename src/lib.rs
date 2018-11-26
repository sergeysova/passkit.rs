extern crate crypto;
extern crate openssl;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod field;
mod pass;
mod personalization;
mod util;

use crypto::{digest::Digest, sha1::Sha1};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::prelude::*;

pub use field::*;
pub use pass::*;
pub use personalization::*;

// use Failure
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PassCreateError {
    CantReadSourceDir,
    CantReadEntry(String),
}

impl fmt::Display for PassCreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PassCreateError::*;
        let stringified = match self {
            CantReadSourceDir => "Can't read source directory".to_string(),
            CantReadEntry(cause) => format!("Can't read {}", cause),
        };
        write!(f, "PassCreateError: {}", stringified)
    }
}

impl std::error::Error for PassCreateError {}

type PassResult<T> = Result<T, PassCreateError>;
type Manifest = HashMap<String, String>;

/// Describes .pass directory with files
#[derive(Debug, Default)]
pub struct PassSource {
    source_directory: String,
    manifest: Manifest,
}

impl PassSource {
    pub fn new<S: Into<String>>(source: S) -> PassSource {
        PassSource {
            source_directory: source.into(),
            ..Default::default()
        }
    }

    pub fn create_pass(&mut self, pass: &Pass) -> PassResult<()> {
        self.manifest = self.create_manifest_hashes()?;
        Ok(())
    }

    fn create_manifest_hashes(&self) -> PassResult<Manifest> {
        let mut manifest = Manifest::new();
        let list =
            fs::read_dir(&self.source_directory).map_err(|_| PassCreateError::CantReadSourceDir)?;

        for entry in list {
            let entry = entry.map_err(|err| PassCreateError::CantReadEntry(err.to_string()))?;
            let content = read_file_to_end(entry.path())
                .map_err(|err| PassCreateError::CantReadEntry(err.to_string()))?;
            let hash = get_hash(&content);
            let file_name = format!("{:?}", entry.file_name());

            println!("— {} >> {}", file_name, hash);
            manifest.insert(file_name, hash);
        }

        Ok(manifest)
    }
}

fn read_file_to_end<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Vec<u8>> {
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
