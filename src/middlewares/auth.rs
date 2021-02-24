use actix_web::{
    dev::HttpResponseBuilder, dev::ServiceRequest, error, get, http::header, http::StatusCode, App,
    Error, HttpResponse,
};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use derive_more::{Display, Error};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::custom;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, custom::MyError> {
    let key = b"secret";
    let decoded = decode::<Claims>(
        &credentials.token(),
        &DecodingKey::from_secret(key),
        &Validation::default(),
    );

    if !decoded.is_err() {
        Ok(req)
    } else {
        let config = req
            .app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default)
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        // Err(AuthenticationError::from(config).into())
        Err(custom::MyError::BadClientData)
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

// pub async fn validator(
//     req: ServiceRequest,
//     credentials: BearerAuth,
// ) -> Result<ServiceRequest, actix_web::Error> {
//     let key = b"secret";
//     let token = credentials.token();
//     let token_data = match decode::<Claims>(
//         &token,
//         &DecodingKey::from_secret(key),
//         &Validation::default(),
//     ) {
//         Ok(c) => c,
//         Err(err) => match *err.kind() {
//             ErrorKind::InvalidToken => panic!(err), // Example on how to handle a specific error
//             _ => panic!(err),
//         },
//     };
//     println!("{:?}", token_data);
//     Ok(req)
// }
