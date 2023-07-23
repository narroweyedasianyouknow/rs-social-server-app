use actix_web::{web, HttpRequest, HttpResponse, Responder};
use hmac::{Hmac, Mac};
use mongodb::{bson::Document, options::InsertOneOptions};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::{
      auth,
      db::get_db,
      types::{
            collections::COLLECTION_NAMES,
            post::{AdditionalItems, MediaItem, MediaTypes, MediumUser, PostCommentUser, PostType},
            user::StoredUserType,
      },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequestBody {
      caption: String,
      children: Option<Vec<MediaItem>>,
      media_type: Option<MediaTypes>,
      media_url: Option<String>,
}

pub async fn main(
      req: HttpRequest,
      post_dto: web::Json<CreatePostRequestBody>,
      authentication: auth::BearerMiddleware,
) -> impl Responder {
      let CreatePostRequestBody {
            caption,
            children,
            media_type,
            media_url,
      } = post_dto.into_inner();

      if media_type.is_none() && children.is_none() {
            HttpResponse::BadRequest().json("{ \"error\": \"error\"}");
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
      HttpResponse::Ok().body("")
}
