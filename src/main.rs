pub mod auth;
pub mod db;
use actix_web::{
      web::{post, resource, Data},
      App, HttpServer,
};
use db::{create_unique_index, get_db};
use dotenv::dotenv;
use routes::posts::like_post;
use routes::{
      authorization::{login, registration},
      posts::create_post,
      uploader::image_upload,
};
mod utils {
      pub mod filter_json;
      pub mod response;
}
use types::{collections::COLLECTION_NAMES, user::StoredUserType};
mod routes {
      pub mod authorization {
            pub mod login;
            pub mod password_utils;
            pub mod registration;
      }
      pub mod posts {
            pub mod create_post;
            pub mod like_post;
      }
      pub mod uploader {
            pub mod image_upload;
      }
}
mod types {
      pub mod collections;
      pub mod post;
      pub mod user;
}

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
      dotenv().ok();

      let url = &env::var("BACKEND_URI").expect("BACKEND_URI not set");
      // Подключение к базе данных
      let db = get_db().await;
      let users_collection: mongodb::Collection<StoredUserType> =
            db.collection(COLLECTION_NAMES.users.as_str());
      let _ = create_unique_index(&users_collection).await;
      HttpServer::new(move || {
            App::new()
                  .route("/registration", post().to(registration::main))
                  .route("/login", post().to(login::main))
                  .service(resource("/upload").route(post().to(image_upload::upload_file)))
                  .route("/post", post().to(create_post::main))
                  .route("/like/{post_id}", post().to(like_post::main))
                  .app_data(Data::new(db.clone()))
      })
      .bind(url)?
      .run()
      .await
}
