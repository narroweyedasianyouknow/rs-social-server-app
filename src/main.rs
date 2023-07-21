pub mod db;

use actix_web::{web, App, HttpServer};
use db::{get_db, create_unique_index};
use dotenv::dotenv;
use routes::authorization::{registration, login};
mod utils {
    pub mod filter_json;
}
use types::{collections::COLLECTION_NAMES, user::StoredUserType};
mod routes {
    pub mod authorization {
        pub mod registration;
        pub mod password_utils;
        pub mod login;
    }
}
mod types {
    pub mod collections;
    pub mod user;
}

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let url = &env::var("BACKEND_URI").expect("BACKEND_URI not set");
    // Подключение к базе данных
    let db = get_db().await;
    let users_collection: mongodb::Collection<StoredUserType> = db.collection(COLLECTION_NAMES.users.as_str());
    let _ = create_unique_index(&users_collection).await;
    HttpServer::new(move || {
        App::new()
            .route("/registration", web::post().to(registration::main))
            .route("/login", web::post().to(login::main))
            // Добавьте свои маршруты и обработчики запросов здесь
            .app_data(web::Data::new(db.clone())) // Добавьте доступ к базе данных в состояние приложения
    })
    .bind(url)?
    .run()
    .await
}
