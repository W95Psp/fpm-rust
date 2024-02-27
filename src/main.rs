#![feature(exit_status_error)]
#![feature(thread_local)]

mod api;

// fn

fn main() {
    use base64::prelude::*;
    use nix_nar::Encoder;
    use sha2::Digest;
    use std::fs::File;
    use std::io;

    let mut enc = Encoder::new("/tmp/FStar").unwrap();
    let mut hasher = sha2::Sha256::new();
    // let mut nar = File::create("/tmp/output.nar").unwrap();
    io::copy(&mut enc, &mut hasher).unwrap();
    // let result: [u8; 16] = hasher.finalize().into();
    let result: [u8; 32] = hasher.finalize().into();
    println!("{:#?}", result.len());
    // println!("sha256-{}", BASE64_STANDARD.encode(result));
}

// mod fstar;
// mod graph;
// mod jobs;
// mod prelude;
// mod source;
// mod try_hash;

// use prelude::*;

// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     use std::collections::hash_map::DefaultHasher;
//     let s = Source::new("/tmp/Hey.fst".into());

//     let deps = tokio::process::Command::fstar().arg(s).dep_graph().await;

//     // println!();
//     println!("Hello, world! {:#?}", deps);
//     // println!("Hello, world! {:x}", s.direct_hash().unwrap());
//     // println!("Hello, world! {:x}", s.direct_hash().unwrap());
//     // println!("Hello, world! {:x}", s.direct_hash().unwrap());
// }
