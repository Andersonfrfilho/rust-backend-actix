use actix_web::{ web, middleware, App, HttpServer};
use actix_service::Service;
use futures::future::FutureExt;
pub mod routes;

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
        .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
        .service(web::scope("/users").configure(routes::users::scoped_config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}