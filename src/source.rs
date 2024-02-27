use once_cell::sync::Lazy;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{hash::Hash, hash::Hasher, io};

use crate::prelude::*;

/// A source file
#[derive(Debug, Clone, Hash, PartialOrd, Ord, Eq, PartialEq)]
pub struct Source {
    path: PathBuf,
}

impl Source {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl AsRef<OsStr> for Source {
    fn as_ref(&self) -> &OsStr {
        self.path.as_ref()
    }
}

impl TryHash for Source {
    type Error = std::io::ErrorKind;
    fn try_hash<H: Hasher>(&self, state: &mut H) -> Result<(), Self::Error> {
        io::copy(
            &mut std::fs::File::open(&self.path).map_err(|e| e.kind())?,
            &mut HashWriter(state),
        )
        .map_err(|e| e.kind())?;
        Ok(())
    }
    fn direct_hash(&self) -> Result<u64, Self::Error> {
        use std::collections::HashMap;
        use std::sync::Mutex;

        static MEMO: Lazy<Mutex<HashMap<Source, Result<u64, std::io::ErrorKind>>>> =
            Lazy::new(|| Mutex::new(HashMap::new()));

        let memo = &mut MEMO.lock().unwrap();
        if let Some(result) = memo.get(self) {
            result.clone()
        } else {
            let result = try_hash::hash(self);
            memo.insert(self.clone(), result);
            result
        }
    }
}
