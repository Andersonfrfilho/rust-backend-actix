use actix_web::{ web, middleware, App, HttpServer };
use actix_service::Service;
use futures::future::FutureExt;
use actix_web_httpauth::{ middleware::HttpAuthentication };
use serde::{ Serialize, Deserialize };
use actix_cors::Cors;
pub mod middlewares;
pub mod routes;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
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
        .service(
          web::scope("/users")
                              .configure(routes::users::scoped_config)
                              .wrap(HttpAuthentication::bearer(middlewares::auth::validator))
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}