use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, FromRequest};
use core::fmt;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::Serialize;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::future::{ready, Ready};

#[derive(Debug, Serialize)]
struct ErrorResponse {
      status: String,
      message: String,
}

impl fmt::Display for ErrorResponse {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", serde_json::to_string(&self).unwrap())
      }
}

pub struct BearerMiddleware {
      pub username: String,
}

impl FromRequest for BearerMiddleware {
      type Error = ActixWebError;
      type Future = Ready<Result<Self, Self::Error>>;

      fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
            let jwt_secret = &std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
            let token = req
                  .headers()
                  .get(http::header::AUTHORIZATION)
                  .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
                  .unwrap();
            let key: Hmac<Sha256> =
                  Hmac::new_from_slice(format!("b\"{}\"", jwt_secret).as_bytes()).unwrap();

            if token.is_empty() {
                  let json_error = ErrorResponse {
                        status: "fail".to_string(),
                        message: "You are not logged in, please provide token".to_string(),
                  };
                  return ready(Err(ErrorUnauthorized(json_error)));
            }

            let claims: BTreeMap<String, String> = token.verify_with_key(&key).unwrap();

            let (_, username) = claims.get_key_value("username").unwrap();

            if username.is_empty() {
                  let json_error = ErrorResponse {
                        status: "fail".to_string(),
                        message: "Invalid token".to_string(),
                  };
                  return ready(Err(ErrorUnauthorized(json_error)));
            }
            ready(Ok(BearerMiddleware {
                  username: username.to_string(),
            }))
      }
}
