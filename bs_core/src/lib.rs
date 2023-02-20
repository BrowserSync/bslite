use actix_web::{middleware, web, App, HttpRequest, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
  println!("REQ: {req:?}");
  "Hello world!"
}

pub fn get_server() -> std::io::Result<actix_web::dev::Server> {
  std::env::set_var("RUST_LOG", "bs_core=debug");
  env_logger::init();

  log::info!("binding to..... http://localhost:8080");

  Ok(
    HttpServer::new(|| {
      App::new()
        // enable logger
        .wrap(middleware::Logger::default())
        .service(web::resource("/index.html").to(|| async { "Hello world!" }))
        .service(web::resource("/").to(index))
    })
    .disable_signals()
    .bind(("127.0.0.1", 8080))?
    .run(),
  )
}

pub fn serve() -> Result<(), String> {
  actix_rt::System::new().block_on(async move {
    match get_server().unwrap().await {
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

#[cfg(test)]
mod tests {
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
}
