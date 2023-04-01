pub mod bind_address;
pub mod browsersync;
pub mod cli;
pub mod server;
pub mod static_route;

pub use crate::bind_address::{BindAddress, BindHostOptions, BindOptions};
use actix_files::Files;
pub use server::Server;

use crate::static_route::{DirPath, RouteResolver};
use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use std::net::TcpListener;
use std::path::PathBuf;


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
    HttpServer::new(move || {
      let mut app = App::new()
        // enable logger
        .wrap(middleware::Logger::default());
      // .wrap_fn(|req, srv| {
      //     let fut = srv.call(req);
      //     async {
      //         let mut res = fut.await?;
      //         dbg!(&res.headers());
      //         // res.headers_mut()
      //         //     .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
      //         Ok(res)
      //     }
      // })
      // .service(Files::new("public", "./assets")
      //     .guard(fn_guard(|f| {
      //       dbg!(&f);
      //       false
      //     }))
      // )
      // .service(Files::new("public", "./assets"))
      // .service(Files::new("public", "./assets2"))

      for route in &server.routes {
        match &route.resolve {
          RouteResolver::RawString(_) => {}
          RouteResolver::FilePath(_) => {}
          RouteResolver::DirPath(DirPath { dir }) => {
            app = app.service(Files::new(&route.path, dir).index_file("index.html"));
          }
        }
      }

      app = app.service(Files::new("/", PathBuf::from(".")).index_file("index.html"));
      app = app.service(web::resource("/index.html").to(|| async { "Hello world!" }));

      app
    })
    .disable_signals()
    .bind((bind_address.ip.as_str(), bind_address.port))?
    .run(),
  )
}

pub fn get_bind_addresses(
  Server {
    bind: bind_address, ..
  }: &Server,
) -> Result<BindAddress, std::io::Error> {
  let as_host = bind_address.ip();
  let check = |num| TcpListener::bind((as_host.as_str(), num));

  let port_choice = match bind_address.port {
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
  use crate::bind_address::{BindHostOptions, BindOptions};
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
      bind: BindOptions {
        port: None,
        host: Some(BindHostOptions::LocalHost),
      },
      routes: vec![],
    };
    let ap = get_bind_addresses(&s);
    println!("{:?}", ap);
  }
}
