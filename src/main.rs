mod try_hash;
use try_hash::*;

use std::path::PathBuf;
use std::{hash::Hash, hash::Hasher, io};

/// A F* source file
pub struct Source {
    path: PathBuf,
}

impl TryHash for Source {
    type Error = std::io::Error;
    fn try_hash<H: Hasher>(&self, state: &mut H) -> Result<(), Self::Error> {
        io::copy(
            &mut std::fs::File::open(&self.path)?,
            &mut HashWriter(state),
        )?;
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {  
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    let s = Source {
        path: "/tmp/hello".into(),
    };
    s.try_hash(&mut hasher).unwrap();
    println!("Hello, world! {:x}", hasher.finish());
}
