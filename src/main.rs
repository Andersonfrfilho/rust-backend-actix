use actix_web::{dev::ServiceRequest,web, middleware, App, HttpServer,Error};
use actix_service::Service;
use futures::future::FutureExt;
use jsonwebtoken::errors::ErrorKind;
use actix_web_httpauth::{
  extractors::bearer::BearerAuth, middleware::HttpAuthentication,
};
use serde::{Serialize,Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use actix_cors::Cors;

pub mod routes;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

async fn validator(
  req: ServiceRequest,
  credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
  let key = b"secret";
  let mut token = credentials.token();
  let token_data = match decode::<Claims>(
    &token,
    &DecodingKey::from_secret(key),
    &Validation::default(),
  ) {
    Ok(c) => c,
    Err(err) => match *err.kind() {
        ErrorKind::InvalidToken => panic!(), // Example on how to handle a specific error
        _ => panic!(),
    },
  };
  println!("{:?}",token_data);
  Ok(req)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
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
        .service(web::scope("/sessions").configure(routes::session::scoped_config))
        .service(web::scope("/users").configure(routes::users::scoped_config).wrap(HttpAuthentication::bearer(validator)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}