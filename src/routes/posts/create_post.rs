use actix_web::{web, HttpRequest, Responder};
use mongodb::{bson::Document, options::InsertOneOptions};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
      auth,
      db::get_db,
      types::{
            collections::COLLECTION_NAMES,
            post::{AdditionalItems, MediaItem, MediaTypes, MediumUser, PostType},
            user::StoredUserType,
      },
      utils::response::request_response,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequestBody {
      caption: String,
      children: Option<Vec<MediaItem>>,
      media_type: Option<MediaTypes>,
      media_url: Option<String>,
}

pub async fn main(
      _req: HttpRequest,
      post_dto: web::Json<CreatePostRequestBody>,
      authentication: auth::BearerMiddleware,
) -> impl Responder {
      let CreatePostRequestBody {
            caption,
            mut children,
            mut media_type,
            mut media_url,
      } = post_dto.into_inner();

      let is_array_of_items = children.is_some();
      let is_single_item = media_type.is_some();

      if is_single_item && media_url.is_none() {
            request_response(true, Some("media_url is required".to_string()), None, None);
      }

      if media_type.is_none() && children.is_none() {
            request_response(
                  true,
                  Some("media_type or children required".to_string()),
                  None,
                  None,
            );
      }
      if is_array_of_items {
            media_type = None;
            media_url = None;
      } else {
            children = None;
      }

      let db = get_db().await;
      let collection: mongodb::Collection<PostType> = db.collection(&COLLECTION_NAMES.post);
      let user_collection: mongodb::Collection<StoredUserType> =
            db.collection(&COLLECTION_NAMES.users);

      let mut document = Document::new();

      document.insert("username", authentication.username);

      let user = user_collection
            .find_one(document, None)
            .await
            .unwrap()
            .unwrap();
      let post_user = MediumUser {
            username: user.username,
            full_name: user.full_name,
            profile_picture: user.profile_picture,
            is_private: user.is_private,
      };
      let insert_options = InsertOneOptions::default();

      let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

      let to_store = PostType {
            caption,
            media_type,
            media_url,
            children,
            thumbnail_url: None,
            likes: AdditionalItems {
                  count: 0,
                  data: Vec::new(),
            },
            comments: AdditionalItems {
                  count: 0,
                  data: Vec::new(),
            },
            timestamp: time.as_secs(),
            is_video: false,
            location: None,
            user: post_user,
      };

      collection
            .insert_one(to_store, insert_options)
            .await
            .unwrap();
      return request_response(
            false,
            Some("Media is uploaded".to_string()),
            Some(201),
            None,
      );
}
