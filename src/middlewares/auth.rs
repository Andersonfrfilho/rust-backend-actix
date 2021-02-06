use actix_web::{dev::ServiceRequest,dev::HttpResponseBuilder,error, get, http::header, http::StatusCode, App, HttpResponse};
use jsonwebtoken::errors::ErrorKind;
use actix_web_httpauth::{
  extractors::bearer::BearerAuth,
};
use serde::{Serialize,Deserialize};
use jsonwebtoken::{decode, Validation, DecodingKey};

use derive_more::{Display, Error};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub async fn validator(
  req: ServiceRequest,
  credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
  let key = b"secret";
  let token = credentials.token();
  let token_data = match decode::<Claims>(
    &token,
    &DecodingKey::from_secret(key),
    &Validation::default(),
  ) {
    Ok(c) => c,
    Err(err) => match *err.kind() {
        ErrorKind::InvalidToken => panic!(err), // Example on how to handle a specific error
        _ => panic!(err),
    },
  };
  println!("{:?}",token_data);
  Ok(req)
}
