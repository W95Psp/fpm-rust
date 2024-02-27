use internment::ArcIntern;
use once_cell::sync::Lazy;
use semver::{Version, VersionReq};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AbsoluteFilePath(PathBuf);
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RelativePath(PathBuf);

// #[derive(Clone, Debug, Eq, PartialEq, Hash)]
// pub struct Sri();

// #[derive(Clone, Debug, Eq, PartialEq, Hash)]
// pub struct LockedUrl(Url);
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackageName(String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Registry(HashMap<PackageName, Url>);

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown")]
    Unknown,
}

pub mod modes {
    use super::*;
    /// Sources, dependencies, etc., are already available on disk
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct Concrete;
    /// Sources, dependencies, etc., might be not downloaded yet
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct Symbolic;

    pub trait Mode: 'static + Eq + Hash + Send + Sync {
        type Source: Clone + Debug + Eq + Hash + Send + Sync;
        type Dependency: Clone + Debug + Eq + Hash + Send + Sync;
    }

    impl Mode for Concrete {
        type Source = AbsoluteFilePath;
        type Dependency = Package<Concrete>;
    }

    impl Mode for Symbolic {
        type Source = RelativePath;
        type Dependency = PackageName;
    }
}

pub mod git {
    use std::hash::Hash;
    use tempfile::tempdir;
    use thiserror::Error;

    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Location<Point: Clone + Eq + Hash> {
        url: String,
        point: Point,
    }
    // pub enum Location<Point: Clone + Eq + Hash> {
    //     Git { url: String, point: Point },
    // }

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("unknown")]
        Unknown,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct GitFullCommit([u8; 20]);
    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    pub enum GitPoint {
        Rev(String),
        Branch(String),
        Tag(String),
    }

    impl GitPoint {
        fn refspec(&self) -> String {
            match self {
                GitPoint::Rev(rev) => rev.clone(),
                GitPoint::Branch(branch) => format!("refs/heads/{}", branch),
                GitPoint::Tag(tag) => format!("refs/tags/{}", tag),
            }
        }
    }

    impl Location<Option<GitPoint>> {
        async fn lock(&self) -> Result<Location<GitFullCommit>, Error> {
            let dir = tempdir().expect("Could not create a temporary directory");
            use tokio::process::Command;
            Command::new("git")
                .current_dir(dir.path())
                .args(["init"])
                .status()
                .await
                .expect("Could not run `git init`")
                .exit_ok()
                .expect("`git init` failed");
            let mut fetch = Command::new("git");
            let refspec = self.point.as_ref().map(GitPoint::refspec);
            fetch
                .current_dir(dir.path())
                .args(["fetch", "--depth=1"])
                .arg(self.url.clone())
                .args(refspec.as_slice());
            fetch
                .status()
                .await
                .expect("Could not run `git fetch`")
                .exit_ok()
                .expect("`git fetch` failed");
            if let Some(refspec) = refspec {
                Command::new("git")
                    .current_dir(dir.path())
                    .arg("checkout")
                    .arg(refspec)
                    .status()
                    .await
                    .expect("Could not run `git checkout`")
                    .exit_ok()
                    .expect("`git fetch` failed");
            }

            Command::new("git")
                .current_dir(dir.path())
                .args(["rev-parse", "HEAD"])
                .output()
                .await
                .expect("Could not run `git rev-parse`")
                .exit_ok()
                .expect("`git init` failed");
            Err(Error::Unknown)
        }
    }
}
use git::*;

pub struct SymbolicDependency {
    name: PackageName,
    url: Url,
    locked_url: LockedUrl,
}

pub struct LockedUrl {
    original: Url,
    locked: Url,
    sri: Sri,
}

pub struct Sri {
    sha256: [u8; 32],
}

use std::fmt;
impl fmt::Display for Sri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use base64::prelude::*;
        write!(f, "sha-{}", BASE64_STANDARD.encode(self.sha256))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Package<M: modes::Mode>(ArcIntern<PackageInner<M>>);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackageInner<M: modes::Mode> {
    name: PackageName,
    dependencies: Vec<M::Dependency>,
    location: Url,
    // hash:
    targets: Vec<Target<M>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TargetKind {
    Lib,
    DunePackage,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Target<M: modes::Mode> {
    name: String,
    kind: TargetKind,
    sources: Vec<M::Source>,
}

#[derive(Clone, Debug)]
pub struct PackageRepo {
    map: HashMap<PackageName, Url>,
}

impl PackageRepo {
    fn load() {}
}

// pub struct PackageId {
//     name: String,
//     version: Version,
//     source_id: SourceId,
// }

// pub struct SourceId {
//     url: Url,
// }

// pub struct Manifest {
//     package_id: PackageId,
//     dependencies: Vec<Dependency>,
//     targets: Vec<Target>,
// }

// pub struct Target {
//     name: String,
//     kind: TargetKind,
//     sources: TargetSource,
// }

// pub enum TargetKind {
//     /// F* library
//     Lib,
//     /// OCaml dune package
//     Dune,
// }

// pub struct Dependency {
//     name: String,
//     version: OptVersionReq,
// }

// pub enum OptVersionReq {
//     Any,
//     Req(VersionReq),
//     Locked {
//         version: Version,
//         original: VersionReq,
//     },
// }

// pub struct Package {
//     manifest: Manifest,
//     manifest_path: PathBuf,
// }

// pub struct Library {

// }

// /// Represents one F* module
// struct FStarSource();

// /// A `*.checked` file
// struct CheckedFile;

// /// Represents the graph of a `fstar.exe --dep ...` run
// struct FStarDependencyGraph;

// impl FStarDependencyGraph {
//     /// Computes the smallest graph capturing all the dependencies of
//     /// the given modules
//     fn shrink(self, modules: Vec<FStarSource>) -> Self {
//         todo!()
//     }

//     /// Call F* to compute the graph of dependencies for [modules]
//     fn new(modules: Vec<FStarSource>, include_directories: Vec<PathBuf>) -> Result<Self, Error> {
//         todo!()
//     }
// }

// enum Codegen {
//     OCaml,
//     FSharp,
//     Krml,
//     Plugin,
// }

// impl FStarSource {
//     /// Run F* and check a module
//     fn check(&self, deps: &FStarDependencyGraph) -> Result<CheckedFile, Error> {
//         todo!()
//     }

//     // /// Extract F*
//     // fn codegen() -> Result<>
// }

// trait Mode {
//     type Modules;
//     type Dependencies;
// }

// struct LibName(String);

// enum LibrarySource {
//     Git { uri: String },
//     Resolved {  }
// }

// pub struct Library<M: Mode> {
//     pub name: LibName,
//     pub modules: M::Modules,
//     pub dependencies: M::Dependencies,
// }

// pub struct LibraryRegistry {
//     pub libs: HashMap<LibName, LibrarySource>,
// }

// impl LibraryRegistry {
//     fn lookup(&self)
// }

// impl FStarSource {
//     fn
// }
