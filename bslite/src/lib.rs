#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::tokio::join;
use std::path::PathBuf;

use napi::bindgen_prelude::*;

#[derive(Debug, serde::Serialize)]
enum Event {
  BindingTo((String, usize)),
}

#[napi]
async fn start(args: Vec<String> /*_func: ThreadsafeFunction<String>*/) -> Result<i32> {
  let handle = spawn(async move {
    // let as_json = serde_json::to_string_pretty(&Event::BindingTo(("127.0.0.1".into(), 8080)))
    //   .expect("can create json for test");

    let mut server = bs_core::Server::default();
    let as_bufs = args.iter().map(PathBuf::from).collect::<Vec<_>>();
    server.dirs = as_bufs;

    // func.call(Ok(as_json), ThreadsafeFunctionCallMode::NonBlocking);

    let server_runner = bs_core::get_server(server).unwrap();

    // let h2 = spawn(async move {
    //   let _ = tokio::signal::ctrl_c().await;
    //   println!("stopping because asked to...");
    //   let l = clone.lock().unwrap();
    // });

    match server_runner.await {
      Ok(_server) => {
        println!("server stopped all done");
      }
      Err(_err) => {
        println!("server stopped from error");
      }
    }
  });

  let _ = join!(handle);
  Ok(32)
}

#[test]
fn test_event_to_json() {
  let as_json = serde_json::to_string_pretty(&Event::BindingTo(("127.0.0.1".into(), 8080)));
  println!("{}", as_json.unwrap());
}
