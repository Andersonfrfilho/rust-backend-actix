use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use serde::{Serialize,Deserialize};
//for response
#[derive(Serialize)]
struct User {
    name: String,
    email: String,
    hash_password: String,

}
async fn response_with_json() -> impl Responder {
    return web::Json(User { 
      name: "anderson".to_string(),
      email:"andersonfrfilho@gmail.com".to_string(),
      hash_password:"102030".to_string()
    })
}
//for request
#[derive(Deserialize)]
struct MyInfo {
    id: String,
    username: String,

}
//for response
#[derive(Serialize)]
struct PathParams{
  id: String,
  username: String,
}
//for response
#[derive(Serialize)]
struct UserExist{
  id: String,
  username: String,
  path: PathParams
}
async fn get_body_path(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
  let path = path.into_inner();
  // format!("{} {} {} {}", path.0, path.1, json.id, json.username)
  let path_params = PathParams {
    id: path.0,
    username: path.1
  };
  return web::Json(UserExist { 
    id: json.id.to_string(),
    username:json.username.to_string(),
    path: path_params,    
  })
}

#[derive(Deserialize,Serialize)]
struct Pagination{
    page: String,
    items: String,
    totals: String
}

async fn get_info_query(info: web::Query<Pagination>) -> impl Responder {
  return web::Json(Pagination { 
    page: info.page.to_string(),
    items: info.items.to_string(),
    totals: info.totals.to_string(),
  })
}

#[derive(Deserialize,Serialize)]
struct Info {
    user_id: u32,
    friend: String,
}
async fn index(info: web::Path<Info>) -> impl Responder {
 return web::Json(Info{
  user_id:info.user_id,
  friend:info.friend.to_string()
 })
}
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::scope("")
          .route("/response/json",web::get().to(response_with_json))
          .route("/params/body/{id}/{username}",web::post().to(get_body_path))
          .route("/query",web::get().to(get_info_query))
          .route("/path/{user_id}/{friend}",web::get().to(index))
          // .route("/{id}",web::put().to(index))
          // .route("/{id}",web::delete().to(index))
  );
}

// pub fn config(cfg: &mut web::ServiceConfig) {
//   cfg.service(
//       web::resource("/users")
//           // .route(web::get().to(|| HttpResponse::Ok().body("app")))
//           // .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
//   );
// }