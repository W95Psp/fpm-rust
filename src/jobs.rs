// use tokio::sync::mpsc;
// use tokio::sync::oneshot;
// use tokio::sync::watch;

// use async_trait::async_trait;
// use std::collections::HashMap;

use std::future::Future;
use tokio::sync::Semaphore;
static PERMITS: Semaphore = Semaphore::const_new(100);
pub async fn spawn_primitive_job<T>(f: impl Future<Output = T>) -> T {
    let _permit = PERMITS.acquire().await.unwrap();
    f.await
}

// #[async_trait]
// trait Job {
//     type Output;
//     async fn run(&self) -> Self::Output;
// }

// struct Goo(Box<dyn Job<Output = u8>>);

// fn spawn<O>(j: dyn Job<Output = O>) {}

// pub enum Msg<J: Job> {
//     Add {
//         job: J,
//         deps: Vec<J>,
//         ondone: Option<oneshot::Sender<()>>,
//     },
//     Finished {
//         job: J,
//         output: Result<(), J::Error>,
//     },
//     // Wait { job: J, ondone: oneshot::Sender<()> },
// }

// trait Job: std::hash::Hash + Eq + Ord + Clone + std::fmt::Debug + Send {
//     type Error: Send;
//     async fn run(&self) -> Result<(), Self::Error>;
// }

// pub enum Msg<J: Job> {
//     Add {
//         job: J,
//         deps: Vec<J>,
//         ondone: Option<oneshot::Sender<()>>,
//     },
//     Finished {
//         job: J,
//         output: Result<(), J::Error>,
//     },
//     // Wait { job: J, ondone: oneshot::Sender<()> },
// }

// #[derive(Clone, Debug)]
// struct Jobs<J: Job> {
//     msg_send: mpsc::UnboundedSender<Msg<J>>,
// }

// struct JobData<J> {
//     ondone_handlers: Vec<oneshot::Sender<()>>,
//     deps: Vec<J>,
// }

// const MAX_JOBS: usize = 10;

// type JobHashMap<J> = HashMap<J, JobData<J>>;

// pub fn sink<J>(graph: &JobHashMap<J>) -> Option<(J, Vec<J>)> {
//     None
// }

// impl<J: Job + 'static> Jobs<J> {
//     pub fn spawn() -> Self {
//         let (msg_send, mut msg_recv) = mpsc::unbounded_channel::<Msg<J>>();

//         tokio::spawn(async move {

//             let mut jobs: JobHashMap<J> = HashMap::new();
//             let mut active_jobs: Vec<J> = vec![];
//             while let Some(msg) = msg_recv.recv().await {
//                 match msg {
//                     Msg::Add { job, deps, ondone } => {
//                         jobs.insert(
//                             job,
//                             JobData {
//                                 deps,
//                                 ondone_handlers: ondone.into_iter().collect(),
//                             },
//                         );
//                     }
//                     Msg::Finished { job, output } => {
//                         if let Some(data) = jobs.remove(&job) {
//                             for send in data.ondone_handlers {
//                                 send.send(());
//                             }
//                         }
//                     }
//                 };
//                 while active_jobs.len() < MAX_JOBS {
//                     if let Some((job, deps)) = sink(&jobs) {
//                         match job.run().await {
//                             Ok(()) => {
//                                 todo!()
//                             }
//                             _ => todo!(),
//                         }
//                     } else {
//                         break;
//                     }
//                 }
//             }
//         });

//         todo!()
//     }
// }
