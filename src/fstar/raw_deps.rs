use crate::prelude::*;

use std::path::PathBuf;
use thiserror::Error;
use tokio::process::Command;

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    UseInterface,
    PreferInterface,
    UseImplementation,
    FriendImplementation,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ModuleKind {
    Implementation,
    Interface,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ModuleName(pub String);
#[derive(Clone, Debug)]
pub struct Edge(pub Kind, pub ModuleName);
#[derive(Clone, Debug)]
pub struct Graph(pub Vec<(Source, Vec<Edge>)>);

impl Graph {
    pub fn sources(&self) -> std::collections::HashSet<Source> {
        self.0.iter().map(|(s, _)| s).cloned().collect()
    }
}

impl ModuleName {
    pub fn infos(s: &Source) -> (Self, ModuleKind) {
        let path = s.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let filename = filename.to_lowercase();
        let fst = filename.strip_suffix(".fst");
        let module = fst.or(filename.strip_suffix(".fsti")).unwrap().to_string();
        (
            ModuleName(module),
            if fst.is_some() {
                ModuleKind::Implementation
            } else {
                ModuleKind::Interface
            },
        )
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Cli(#[from] super::cli::Error),
    #[error("Error while parsing F*'s deps format: {0}")]
    DepsParse(String),
}

#[extension_traits::extension(pub trait ExtCommandFStarCliDeps)]
impl Command {
    async fn raw_deps(&mut self) -> Result<Graph, Error> {
        self.args(["--dep", "raw"]);
        let (stdout, _) = spawn_primitive_job(self.output_str()).await?;
        let r = Graph::parser(&stdout).map_err(|_| Error::DepsParse(stdout.clone()))?;
        Ok(r.1)
    }
}

const _: () = {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        bytes::complete::take_until,
        bytes::complete::take_while1,
        character::complete::char,
        combinator::map,
        multi::separated_list0,
        sequence::{delimited, separated_pair, terminated, tuple},
        IResult,
    };

    impl Kind {
        fn parser(input: &str) -> IResult<&str, Self> {
            alt((
                map(tag("UseInterface"), |_| Self::UseInterface),
                map(tag("PreferInterface"), |_| Self::PreferInterface),
                map(tag("UseImplementation"), |_| Self::UseImplementation),
                map(tag("FriendImplementation"), |_| Self::FriendImplementation),
            ))(input)
        }
    }

    impl ModuleName {
        fn parser(input: &str) -> IResult<&str, Self> {
            map(take_while1(|c: char| c != ';' && c != '\n'), |s: &str| {
                Self(s.to_string())
            })(input)
        }
    }

    impl Edge {
        fn parser(input: &str) -> IResult<&str, Self> {
            map(
                separated_pair(Kind::parser, char(' '), ModuleName::parser),
                |(kind, module_name)| Edge(kind, module_name),
            )(input)
        }
        fn parser_n(input: &str) -> IResult<&str, Vec<Self>> {
            separated_list0(tag(";\n\t"), Self::parser)(input)
        }
    }

    fn parse_source_edge(input: &str) -> IResult<&str, (Source, Vec<Edge>)> {
        fn header(input: &str) -> IResult<&str, &str> {
            const HEADER_END: &str = " -> [\n";
            terminated(take_until(HEADER_END), tag(HEADER_END))(input)
        }
        tuple((
            map(header, |s| Source::new(s.into())),
            delimited(char('\t'), Edge::parser_n, tag("\n] ")),
        ))(input)
    }

    impl Graph {
        pub fn parser(input: &str) -> IResult<&str, Self> {
            let mut n = map(separated_list0(tag(";;\n"), parse_source_edge), Graph);
            terminated(n, terminated(char('\n'), nom::combinator::eof))(input)
        }
    }
};
