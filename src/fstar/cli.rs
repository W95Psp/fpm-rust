use std::future::Future;
use std::process::{ExitStatus, Stdio};
use thiserror::Error;
use tokio::process::Command;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error while running F*: {0}")]
    IO(#[from] std::io::Error),
    #[error("Unknown error while running F*: {0:#?}")]
    Unknown(OutputStr),
}

// use petgraph::Graph;

// pub struct DependencyGraph(Graph<crate::Source, ()>);

#[derive(Clone, Debug)]
pub struct OutputStr {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

#[extension_traits::extension(pub trait ExtCommandFStarCli)]
impl Command {
    fn fstar() -> Self {
        Self::new("fstar.exe")
    }

    async fn output_str_raw(&mut self) -> Result<OutputStr, std::io::Error> {
        let out = self
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        Ok(OutputStr {
            stdout: String::from_utf8(out.stdout).unwrap(),
            stderr: String::from_utf8(out.stderr).unwrap(),
            status: out.status,
        })
    }

    async fn output_str(&mut self) -> Result<(String, String), Error> {
        let out = self.output_str_raw().await?;
        if out.status.success() {
            Ok((out.stdout, out.stderr))
        } else {
            Err(Error::Unknown(out))
        }
    }
}
