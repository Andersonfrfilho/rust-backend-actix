use actix_web::{web,http,error,Result, Responder,HttpRequest, HttpResponse};
use jsonwebtoken::errors::ErrorKind;
use serde::{Serialize,Deserialize};
use derive_more::{Display, Error};
use actix_session::{Session};
use log::debug;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

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
struct CreateSession {
    email: String,
    password: String,

}
//for response
#[derive(Serialize)]
struct SessionCreated{
  email: String,
  token: String,
  token_data: Claims
}
async fn get_body_path(json: web::Json<CreateSession>) -> impl Responder {
  let mut header = Header::new(Algorithm::HS512);
  let key = b"secret";
  let my_claims =
      Claims { sub: "b@b.com".to_owned(), company: "ACME".to_owned(), exp: 10000000000 };
  let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
    Ok(t) => t,
    Err(_) => panic!(), // in practice you would return the error
  };
  let validation = Validation { sub: Some("b@b.com".to_string()), ..Validation::default() };
  let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
    Ok(c) => c,
    Err(err) => match *err.kind() {
        ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
        ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
        _ => panic!("Some other errors"),
    },
  };
  return web::Json(SessionCreated { 
    email: json.email.to_string(),
    token: token.to_string(),
    token_data: token_data.claims
  })
}

// #[derive(Deserialize,Serialize)]
// struct Pagination{
//     page: String,
//     items: String,
//     totals: String
// }

// async fn get_info_query(info: web::Query<Pagination>) -> impl Responder {
//   return web::Json(Pagination { 
//     page: info.page.to_string(),
//     items: info.items.to_string(),
//     totals: info.totals.to_string(),
//   })
// }

// #[derive(Deserialize,Serialize)]
// struct Info {
//     user_id: u32,
//     friend: String,
// }
// async fn path_with_struct(info: web::Path<Info>) -> impl Responder {
//  return web::Json(Info{
//   user_id:info.user_id,
//   friend:info.friend.to_string()
//  })
// }

// #[derive(Debug, Display, Error)]
// #[display(fmt = "my error: {}", name)]
// pub struct MyError {
//     name: &'static str,
// }
// //prepared error
// impl error::ResponseError for MyError {}

// async fn index_custom_error() -> Result<&'static str, MyError> {
//   let err = MyError { name: "test error" };
//   debug!("{}", err);
//   Err(err)
// }

// async fn index_using_session(session: Session) -> Result<HttpResponse, actix_web::Error> {
//   // access session data
//   if let Some(count) = session.get::<i32>("counter")? {
//       session.set("counter", count + 1)?;
//   } else {
//       session.set("counter", 1)?;
//   }

//   Ok(HttpResponse::Ok().body(format!(
//       "Count is {:?}!",
//       session.get::<i32>("counter")?.unwrap()
//   )))
// }

// async fn index(req: HttpRequest) -> Result<fs::NamedFile, actix_web::Error> {
//   let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
//   let file = fs::NamedFile::open(path)?;
//   Ok(file
//       .use_last_modified(true)
//       .set_content_disposition(ContentDisposition {
//           disposition: DispositionType::Attachment,
//           parameters: vec![],
//       }))
// }

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::scope("")
          .route("",web::post().to(get_body_path))
  );
}

// pub fn config(cfg: &mut web::ServiceConfig) {
//   cfg.service(
//       web::resource("/users")
//           // .route(web::get().to(|| HttpResponse::Ok().body("app")))
//           // .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
//   );
// }

#[allow(dead_code)]
async fn index_for_test(req: HttpRequest) -> HttpResponse {
  if let Some(_hdr) = req.headers().get(http::header::CONTENT_TYPE) {
      HttpResponse::Ok().into()
  } else {
      HttpResponse::BadRequest().into()
  }
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = index_for_test(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index_for_test(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_for_test))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_index_post() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_for_test))).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }
}

