use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    db::get_db,
    types::{
        collections::COLLECTION_NAMES,
        post::{MediaTypes, PostType, MediaItem},
    },
};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequestBody {
    caption: String,
    children: Option<Vec<MediaItem>>,
    media_type: Option<MediaTypes>,
    media_url: Option<String>,
}

pub async fn main(post_dto: web::Json<CreatePostRequestBody>) -> impl Responder {
    let CreatePostRequestBody {
        caption,
        children,
        media_type,
        media_url,
    } = post_dto.into_inner();

    let to_store = PostType {
        caption,
        media_type,
        media_url,
        children,
        thumbnail_url: todo!(),
        permalink: todo!(),
        likes: todo!(),
        comments: todo!(),
        timestamp: todo!(),
        is_video: todo!(),
        location: todo!(),
        user: todo!(),
    };
    let collection: mongodb::Collection<PostType> =
        get_db().await.collection(&COLLECTION_NAMES.post);
    HttpResponse::Ok().body("ok")
}
