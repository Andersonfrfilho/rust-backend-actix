use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(web::scope("/users").configure(routes::users::scoped_config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}