use actix_web::{ web, middleware::Logger, App, HttpServer};

pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new().wrap(Logger::default())
        .service(web::scope("/users").configure(routes::users::scoped_config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}