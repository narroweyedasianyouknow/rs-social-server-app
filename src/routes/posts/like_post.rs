use crate::{
      auth,
      db::get_db,
      types::{
            collections::COLLECTION_NAMES,
            post::{MinimalUser, PostType},
            user::StoredUserType,
      },
      utils::response::request_response,
};
use actix_web::{HttpRequest, Responder};
use mongodb::{
      bson::doc,
      options::{FindOneAndUpdateOptions, FindOneOptions},
};

pub async fn main(req: HttpRequest, authentication: auth::BearerMiddleware) -> impl Responder {
      let (_path, post_id) = req.path().split_at(6);

      if post_id.is_empty() {
            return request_response(true, Some("post_id is required".to_string()), None, None);
      }
      let db = get_db().await;
      let collection: mongodb::Collection<PostType> = db.collection(&COLLECTION_NAMES.post);
      let user_collection: mongodb::Collection<StoredUserType> =
            db.collection(&COLLECTION_NAMES.users);

      let object_id = bson::oid::ObjectId::parse_str(&post_id).unwrap();

      let StoredUserType {
            username,
            profile_picture,
            ..
      } = user_collection
            .find_one(
                  doc! {
                        "username": authentication.username
                  },
                  FindOneOptions::default(),
            )
            .await
            .unwrap()
            .unwrap();

      let user = MinimalUser {
            username,
            profile_picture,
      };
      // Add Like From DB
      let filter = doc! {
            "_id": object_id,
            "likes.data": {
                  // Check If user doesn't already liked it DB
                  "$not": {
                        "$elemMatch": bson::to_bson(&user).unwrap()
                  }
            }
      };
      let update = doc! {
           "$inc": { "likes.count": 1 },
           "$addToSet": { "likes.data": bson::to_bson(&user).unwrap() },
      };
      let options = FindOneAndUpdateOptions::builder().upsert(false).build();

      let updated_post = collection
            .find_one_and_update(filter, update, options.clone())
            .await;
      if updated_post.unwrap().is_none() {
            // Remove Like From DB
            let filter = doc! {
                  "_id": object_id,
            };
            let update = doc! {
                 "$inc": { "likes.count": -1 },
                 "$pull": { "likes.data": bson::to_bson(&user).unwrap() },
            };
            let _ = collection
                  .find_one_and_update(filter, update, options)
                  .await;
      }
      return request_response(false, None, None, None);
}
