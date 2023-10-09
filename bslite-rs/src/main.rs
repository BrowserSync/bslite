use bs_core::browsersync::Browsersync;
use bs_core::cli::Cli;
use std::env::current_dir;
use std::process;
use std::vec::IntoIter;

#[tokio::main]
async fn main() {
  let args = std::env::args().into_iter().collect::<Vec<String>>();

  let cli = Cli::from_args(&args);
  let cwd = current_dir().expect("must access current DIR");

  let Ok(cli) = cli else {
    let e = cli.unwrap_err();
    eprintln!("{}", e);
    return;
  };

  let bs = Browsersync::from_cli(cli, cwd);
  let bs_server = bs.base_server().clone();
  let routes_clone = bs_server.routes.clone();
  let routes_clone_2 = bs.routes.clone();
  let (actix_server, bind_address) = bs_core::get_server(bs_server, bs.routes).unwrap();

  // println!("{}", bind_address);

  for sr in &routes_clone {
    println!("{}{}: {}", bind_address, sr.path, sr.resolve);
  }

  for sr in &routes_clone_2 {
    println!("{}{}: {}", bind_address, sr.path, sr.resolve);
  }

  process::exit(match actix_server.await {
    Ok(_server) => {
      println!("server stopped all done");
      0
    }
    Err(_err) => {
      println!("server stopped from error");
      1
    }
  })
}
