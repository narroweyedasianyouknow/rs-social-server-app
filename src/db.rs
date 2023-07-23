use std::env;
use std::error::Error;

use mongodb::{
      options::{ClientOptions, IndexOptions},
      Client, Collection, Database, IndexModel,
};

use mongodb::bson::doc;

use crate::types::user::StoredUserType;

pub async fn get_db() -> Database {
      let client_options =
            ClientOptions::parse(&env::var("MONGO_DB_URL").expect("MONGODB_URL not set"))
                  .await
                  .expect("Failed to parse MongoDB options");
      let client =
            Client::with_options(client_options).expect("Failed to initialize MongoDB client");
      client.database(&env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME not set"))
}
pub async fn create_unique_index(
      collection: &Collection<StoredUserType>,
) -> Result<(), Box<dyn Error>> {
      let index_model = IndexModel::builder()
            .keys(doc! {"username": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

      collection.create_index(index_model, None).await?;

      Ok(())
}
