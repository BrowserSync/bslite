pub mod bind_address;
pub mod browsersync;
pub mod cli;
pub mod server;
pub mod static_route;

pub use crate::bind_address::{BindAddress, BindHostOptions, BindOptions};
use actix_files::{Files, NamedFile};
pub use server::BsServer;

use crate::static_route::{DirPath, FilePath, RawString, RouteResolver, StaticRoute};
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::http::Method;
use std::sync::Arc;

async fn raw_reader(server: Data<Arc<BsServer>>, req: HttpRequest) -> HttpResponse {
  // dbg!(req.match_info());
  // dbg!(req.match_name());
  // dbg!(req.match_pattern());

  let route = server
    .get_ref()
    .routes
    .iter()
    .find(|s| s.path == req.match_pattern().unwrap());

  let route = route.unwrap();

  match &route.resolve {
    RouteResolver::RawString(RawString { raw, headers }) => {
      let mut res = HttpResponse::Ok().body(raw.clone());
      let h = res.headers_mut();
      for (k, v) in headers.iter() {
        h.append(
          HeaderName::from_bytes(k.as_bytes()).unwrap(),
          HeaderValue::from_bytes(v.as_bytes()).unwrap(),
        );
      }
      res
    }
    RouteResolver::FilePath(FilePath { headers, file }) => {
      let h_m = Box::new(headers.clone());
      let named_file = NamedFile::open_async(file).await.unwrap();
      let mut res = named_file.into_response(&req);
      let h = res.headers_mut();
      for (k, v) in h_m.iter() {
        h.append(
          HeaderName::from_bytes(k.as_bytes()).unwrap(),
          HeaderValue::from_bytes(v.as_bytes()).unwrap(),
        );
      }
      res
      // HttpResponse::NotImplemented().body("n")
    }
    RouteResolver::DirPath(_) => HttpResponse::NotImplemented().body("n"),
  }
}

pub fn get_server(
  server: BsServer,
  routes: Vec<StaticRoute>,
) -> std::io::Result<(actix_web::dev::Server, BindAddress)> {
  std::env::set_var("RUST_LOG", "bs_core=debug");
  env_logger::init();
  let bind_address = get_bind_addresses(&server)?;
  let route_arc = Data::new(Arc::new(server.clone()));

  Ok((
    HttpServer::new(move || {
      let mut app = App::new()
        .app_data(route_arc.clone())
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

      for route in server.routes.iter().chain(routes.iter()) {
        match &route.resolve {
          RouteResolver::RawString(RawString { .. }) => {
            app = app.service(web::resource(&route.path).route(web::route().to(raw_reader)));
          }
          RouteResolver::FilePath(FilePath { .. }) => {
            app = app.service(web::resource(&route.path).route(web::route().to(raw_reader)));
          }
          RouteResolver::DirPath(DirPath { dir }) => {
            app = app.service(Files::new(&route.path, dir).index_file("index.html"));
          }
        }
      }

      app
    })
    .disable_signals()
    .bind((bind_address.ip.as_str(), bind_address.port))?
    .run(),
    bind_address,
  ))
}

pub fn get_bind_addresses(
  BsServer {
    bind: bind_address, ..
  }: &BsServer,
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
    let s = BsServer {
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

async fn index() -> impl Responder {
  NamedFile::open_async("./static/index.html").await.unwrap()
}
