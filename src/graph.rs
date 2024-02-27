use crate::prelude::*;
use crate::{fstar::raw_deps::Edge, prelude::*};
use std::collections::{HashMap, HashSet};
use std::future::Future;
use tokio::process::Command;

#[derive(Clone, Debug)]
pub struct Graph(HashMap<Source, HashSet<Source>>);

#[derive(Debug, Error)]
pub enum Error {
    #[error("Duplicated key")]
    DuplicatedModules(#[from] DuplicatedModulesError),
    #[error("Err")]
    RawDepsError(#[from] fstar::raw_deps::Error),
}

#[derive(Debug, Error)]
#[error("dups")]
pub struct DuplicatedModulesError(pub Vec<HashSet<Source>>);

impl TryFrom<fstar::raw_deps::Graph> for Graph {
    type Error = DuplicatedModulesError;
    fn try_from(graph: fstar::raw_deps::Graph) -> Result<Graph, Self::Error> {
        use fstar::raw_deps::{Kind, ModuleKind, ModuleName};
        let sources: HashMap<(ModuleName, ModuleKind), Source> = {
            let mut sources = HashMap::new();
            let mut dups: HashMap<String, HashSet<Source>> = HashMap::new();
            for source in graph.sources() {
                let name = ModuleName::infos(&source);
                if let Some(other_source) = sources.insert(name.clone(), source.clone()) {
                    let set = dups.entry(name.0 .0).or_insert(HashSet::new());
                    set.insert(other_source);
                    set.insert(source);
                }
            }
            if dups.is_empty() {
                sources
            } else {
                return Err(DuplicatedModulesError(dups.into_values().collect()));
            }
        };

        let mut g: HashMap<Source, HashSet<Source>> = HashMap::new();

        for (ref x, edges) in graph.0 {
            let entry = g.entry(x.clone()).or_insert(HashSet::new());
            for Edge(kind, module) in edges {
                let interface = sources.get(&(module.clone(), ModuleKind::Interface));
                let implementation = sources.get(&(module.clone(), ModuleKind::Implementation));
                let y = match kind {
                    Kind::UseInterface => interface,
                    Kind::UseImplementation | Kind::FriendImplementation => implementation,
                    Kind::PreferInterface => interface.or(implementation),
                };
                entry.insert(y.unwrap().clone());
            }
        }

        Ok(Graph(g))
    }
}

// impl Graph {
//     fn narrow(&self) -> Graph {

//     }
//     fn check(&self){

//     }
// }

// use petgraph::prelude::NodeIndex;
// impl Graph {
//     fn petgraph(graph: &Graph) -> (petgraph::Graph<Source, ()>, HashMap<NodeIndex, Source>) {
//         let mut g: petgraph::Graph<Source, ()> = petgraph::Graph::new();
//         let mut nodes: HashMap<Source, NodeIndex> = HashMap::new();
//         for source in graph.0.keys().cloned() {
//             nodes.entry(source.clone()).or_insert(g.add_node(source));
//         }

//         for (x, edges) in &graph.0 {
//             let x = nodes.get(x).unwrap();
//             for y in edges {
//                 let y = nodes.get(y).unwrap();
//                 g.add_edge(*x, *y, ());
//             }
//         }

//         (g, nodes.into_iter().map(|(k, v)| (v, k)).collect())
//     }
// }

// impl Graph {
//     fn get_sink(&self) -> Option<Source> {
//         let (g, nodes) = Graph::petgraph(self);
//         g.externals(petgraph::Direction::Incoming)
//             .next()
//             .map(|i| nodes.get(&i).unwrap())
//             .cloned()
//     }

//     async fn run_jobs<F: Future<Output = ()> + 'static>(f: impl Fn(Source, HashSet<Source>) -> F) {
//         const MAX_JOBS: u32 = 4;
//         let mut jobs: Vec<F> = vec![];
//         ()
//     }
// }

#[extension_traits::extension(pub trait ExtCommandFStarDepGraph)]
impl Command {
    async fn dep_graph(&mut self) -> Result<Graph, Error> {
        let graph = self.raw_deps().await?;
        let graph: Graph = graph.try_into()?;
        Ok(graph)
    }
}

fn test() {

    // // let x: NodeIndex = graph.add_node(todo!());
    // // let nodes:

    // // let sources = graph.sources();
    // // let graph: HashMap<Source, Source> = ();

    // todo!()
}
