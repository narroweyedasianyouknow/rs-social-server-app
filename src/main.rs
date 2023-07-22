pub mod db;

use actix_web::{
    web::{post, resource, Data},
    App, HttpServer,
};
use db::{create_unique_index, get_db};
use dotenv::dotenv;
use routes::{
    authorization::{login, registration},
    posts::create_post,
    uploader::image_upload,
};
mod utils {
    pub mod filter_json;
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
            .service(resource("/upload").route(post().to(image_upload::upload_file)))
            .route("/registration", post().to(registration::main))
            .route("/post", post().to(create_post::main))
            .route("/login", post().to(login::main))
            // Добавьте свои маршруты и обработчики запросов здесь
            .app_data(Data::new(db.clone())) // Добавьте доступ к базе данных в состояние приложения
    })
    .bind(url)?
    .run()
    .await
}
