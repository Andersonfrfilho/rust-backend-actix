use actix_web::{dev::ServiceRequest,web, middleware, App, HttpServer,Error};
use actix_service::Service;
use futures::future::FutureExt;
use actix_web_httpauth::{
  extractors::bearer::BearerAuth, middleware::HttpAuthentication,
};

use actix_cors::Cors;

pub mod routes;

async fn validator(
  req: ServiceRequest,
  credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
  let mut token = credentials.token();
  
  println!("{:?}",token);
  Ok(req)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .wrap(HttpAuthentication::bearer(validator))
        .wrap(middleware::Logger::default())
        .wrap(middleware::Logger::new("%a %{User-Agent}i"))
        .wrap_fn(|req, srv| {
          println!("Hi from start. You requested: {}", req.path());
          srv.call(req).map(|res| {
              println!("Hi from response");
              res
          })
        })
        .wrap(Cors::permissive())
        .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
        .service(web::scope("/users").configure(routes::users::scoped_config))
        .service(web::scope("/sessions").configure(routes::session::scoped_config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}