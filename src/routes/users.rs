use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
  "Hello world! - 2"
}

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::scope("")
          .route("",web::get().to(index))
          .route("",web::post().to(index))
          .route("/{id}",web::get().to(index))
          .route("/{id}",web::put().to(index))
          .route("/{id}",web::delete().to(index))
  );
}

// pub fn config(cfg: &mut web::ServiceConfig) {
//   cfg.service(
//       web::resource("/users")
//           // .route(web::get().to(|| HttpResponse::Ok().body("app")))
//           // .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
//   );
// }