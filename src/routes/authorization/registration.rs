use crate::get_db;
use crate::routes::authorization::password_utils::hash_password;
use crate::types::collections::COLLECTION_NAMES;
use crate::types::user::{RegisterDto, StoredUserType};
use crate::utils::response::request_response;
use actix_web::cookie::{self, Cookie};
use actix_web::{web, HttpResponse, Responder};

use cookie::time::Duration;
use mongodb::{options::InsertOneOptions, Collection, Database};
use serde::{Deserialize, Serialize};
use std::env;
use validator::Validate;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
      code: i32,
      error: String,
}

pub async fn main(user_dto: web::Json<RegisterDto>) -> impl Responder {
      let is_valid = user_dto.validate();

      if is_valid.is_err() {
            return HttpResponse::BadRequest().json(is_valid.err());
      }
      let RegisterDto {
            email,
            full_name,
            password,
            username,
      } = user_dto.into_inner();

      let hashed_password = hash_password(&password).expect("Failed to hash password.");
      let to_store = StoredUserType {
            bio: None,
            email: Some(email.to_string()),
            followers: 0,
            following: 0,
            full_name: Some(full_name.to_string()),
            is_private: false,
            is_verified: false,
            password: hashed_password,
            phone: None,
            posts: 0,
            username,
            website: None,
            profile_picture: None,
      };
      if password.is_empty() {
            return request_response(true, Some("Missing password".to_string()), None, None);
      }

      let db: Database = get_db().await;

      let users_collection_name = &COLLECTION_NAMES.users;

      let collection: Collection<StoredUserType> = db.collection(users_collection_name);
      let insert_options = InsertOneOptions::default();

      let backend_uri = &env::var("BACKEND_URI").expect("BACKEND_URI not set");
      let jwt_secret = &env::var("JWT_SECRET").expect("JWT_SECRET not set");

      let key: Hmac<Sha256> =
            Hmac::new_from_slice(format!("b\"{}\"", jwt_secret).as_bytes()).unwrap();
      let mut claims = BTreeMap::new();
      claims.insert("username", to_store.username.clone());
      let token_str = claims.sign_with_key(&key).unwrap();

      let cookie = Cookie::build("authorization", format!("Bearer {}", token_str))
            .domain(backend_uri)
            .path("/")
            .secure(true)
            .http_only(true)
            .max_age(Duration::days(7))
            .finish();
      if let Err(_) = collection.insert_one(to_store, insert_options).await {
            return request_response(true, Some("Try later".to_string()), None, None);
      }

      return request_response(false, Some("ok".to_string()), Some(201), Some(cookie));
}
