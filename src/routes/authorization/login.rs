use std::env;

use actix_web::{
      cookie::{time::Duration, Cookie},
      web, HttpResponse, Responder,
};
use mongodb::{bson::Document, Collection};
use validator::Validate;

use crate::{
      db::get_db,
      types::{
            collections::COLLECTION_NAMES,
            user::{LoginDto, StoredUserType},
      },
      utils::response::request_response,
};

use super::password_utils::verify_password;

pub async fn main(user_dto: web::Json<LoginDto>) -> impl Responder {
      let is_valid = user_dto.validate();

      if is_valid.is_err() {
            return HttpResponse::BadRequest().json(is_valid.err());
      }
      let mut document = Document::new();
      let user = user_dto.into_inner();

      if user.email.is_some() {
            document.insert("email", user.email);
      } else if user.username.is_some() && !document.contains_key("email") {
            document.insert("username", user.username);
      } else {
            return HttpResponse::BadRequest().body("Bad request");
      }

      let users_collection_name = &COLLECTION_NAMES.users;
      let collection: Collection<StoredUserType> = get_db().await.collection(users_collection_name);

      match collection.find_one(document, None).await {
            Ok(Some(document)) => {
                  let hashed_password = verify_password(&user.password, &document.password);
                  if !hashed_password {
                        return request_response(
                              true,
                              Some("Password or login doesn't correct".to_string()),
                              Some(400),
                              None,
                        );
                  }
                  let backend_uri = &env::var("BACKEND_URI").expect("BACKEND_URI not set");

                  let cookie = Cookie::build("authorization", &document.password)
                        .domain(backend_uri)
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .max_age(Duration::days(7))
                        .finish();

                  request_response(
                        false,
                        Some("Successfully logged in".to_string()),
                        Some(200),
                        Some(cookie),
                  )
            }
            Ok(None) => {
                  request_response(true, Some("Failed to login".to_string()), Some(403), None)
            }
            Err(_) => request_response(true, Some("Failed to login".to_string()), Some(403), None),
      }
}
