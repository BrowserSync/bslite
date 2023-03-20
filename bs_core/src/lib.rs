pub mod bind_address;
pub mod browsersync;

pub use crate::bind_address::{BindAddress, BindAddressOptions, BindHostOptions};
pub use crate::browsersync::Server;
use actix_files::Files;

use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use std::net::TcpListener;

async fn index(req: HttpRequest) -> &'static str {
  println!("REQ: {req:?}");
  "Hello world!"
}

pub fn get_server(server: Server) -> std::io::Result<actix_web::dev::Server> {
  std::env::set_var("RUST_LOG", "bs_core=debug");
  env_logger::init();
  let bind_address = get_bind_addresses(&server)?;

  println!("binding to {bind_address}");

  Ok(
    HttpServer::new(|| {
      App::new()
        // enable logger
        .service(Files::new("/", ".").index_file("index.html"))
        .service(web::resource("/index.html").to(|| async { "Hello world!" }))
        .wrap(middleware::Logger::default())
        .service(web::resource("/").to(index))
    })
    .disable_signals()
    .bind((bind_address.ip.as_str(), bind_address.port))?
    .run(),
  )
}

pub fn serve() -> Result<(), String> {
  actix_rt::System::new().block_on(async move {
    match get_server(Default::default()).unwrap().await {
      Ok(_) => {
        println!("all done");
      }
      Err(_) => {
        println!("oops");
      }
    };
    Ok(())
  })
}

pub fn get_bind_addresses(Server { bind_address }: &Server) -> Result<BindAddress, std::io::Error> {
  let as_host = bind_address.ip();
  let check = |num| TcpListener::bind((as_host.as_str(), num));

  let port_choice = match bind_address.port_preference {
    None => check(0),
    // if a preference was given, use it - or default to 0 for 'any'
    Some(port) => check(port).or_else(|_| check(0)),
  };

  port_choice
    .and_then(|v| v.local_addr())
    .map(|v| BindAddress::from_socket_addr(v))
}

#[cfg(test)]
mod tests {
  use crate::bind_address::{BindAddressOptions, BindHostOptions};
  use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};

  use super::*;

  #[actix_web::test]
  async fn test_index() -> Result<(), Error> {
    let app = App::new().route("/", web::get().to(index));
    let app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = app.call(req).await?;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let response_body = resp.into_body();
    assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

    Ok(())
  }

  #[test]
  async fn test_port() {
    let s = Server {
      bind_address: BindAddressOptions {
        port_preference: None,
        host: Some(BindHostOptions::LocalHost),
      },
    };
    let ap = get_bind_addresses(&s);
    println!("{:?}", ap);
  }
}
