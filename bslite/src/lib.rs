#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::threadsafe_function::{ThreadsafeFunction};
use napi::tokio::join;

use napi::bindgen_prelude::*;

#[derive(Debug, serde::Serialize)]
enum Event {
  BindingTo((String, usize)),
}

#[napi]
async fn start(_a: i32, _func: ThreadsafeFunction<String>) -> Result<i32> {
  let handle = spawn(async move {
    // let as_json = serde_json::to_string_pretty(&Event::BindingTo(("127.0.0.1".into(), 8080)))
    //   .expect("can create json for test");
    let server = bs_core::Server {
      bind_address: bs_core::BindAddressOptions {
        port_preference: Some(3210),
        host: None,
      },
    };

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
