#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use bs_core::browsersync::Browsersync;
use bs_core::cli::Cli;
use clap::Parser;
use napi::bindgen_prelude::*;
use napi::tokio::join;
use std::env::current_dir;

#[derive(Debug, serde::Serialize)]
enum Event {
  BindingTo((String, usize)),
}

#[napi]
async fn start(args: Vec<String> /*_func: ThreadsafeFunction<String>*/) -> Result<i32> {
  let handle = spawn(async move {
    // let as_json = serde_json::to_string_pretty(&Event::BindingTo(("127.0.0.1".into(), 8080)))
    //   .expect("can create json for test");
    let cwd = current_dir().expect("must access current DIR");
    let cli = Cli::try_parse_from(&args);
    let Ok(cli) = cli else {
      let e = cli.unwrap_err();
      eprintln!("{}", e);
      return;
    };

    dbg!(&args);
    dbg!(&cli);

    let bs = Browsersync::from_cli(cli, cwd);
    let server = bs.base_server().clone();
    let routes = server.routes.clone();
    let (actix_server, bind_address) = bs_core::get_server(server, bs.routes).unwrap();

    for sr in routes {
      println!("{} : {}", sr.path, sr.resolve);
    }

    println!("{bind_address}");

    match actix_server.await {
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
