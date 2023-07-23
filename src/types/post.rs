use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Тип MediumUser
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MediumUser {
      pub username: String,
      pub full_name: Option<String>,
      pub profile_picture: Option<String>,
      pub is_private: bool,
}

// Тип MinimalUser
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MinimalUser {
      pub username: String,
      pub profile_picture: Option<String>,
}

// Тип PostCommentUser
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PostCommentUser {
      pub id: String,
      pub text: String,
      pub author: MinimalUser,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MediaItem {
      media_type: MediaTypes,
      media_url: String,
      thumbnail_url: String,
}
// Тип AdditionalItems
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AdditionalItems<T> {
      /// (число): Общее количество, полученных постом.
      pub count: i32,

      /// (массив): Массив объектов, содержащих информацию о пользователях.
      pub data: Vec<T>,
}

// Тип Location
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Location {
      /// (строка): Уникальный идентификатор местоположения.
      pub id: String,

      /// (строка): Название местоположения.
      pub name: String,

      /// (число): Широта местоположения.
      pub latitude: f64,

      /// (число): Долгота местоположения.
      pub longitude: f64,

      /// (строка): Адрес местоположения или его описание.
      pub address: String,

      /// (строка): Название города, связанного с местоположением.
      pub city: String,

      /// (строка): Название страны, связанной с местоположением.
      pub country: String,
}

// Тип PostWithId
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PostWithId {
      /// Уникальный идентификатор поста.   
      #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
      pub id: Option<ObjectId>,
      // Остальные поля, взятые из типа PostType
      pub caption: String,
      pub media_type: Option<MediaTypes>,
      pub media_url: Option<String>,
      pub children: Option<Vec<MediaItem>>,
      pub thumbnail_url: Option<String>,
      pub permalink: String,
      pub likes: AdditionalItems<MinimalUser>,
      pub comments: AdditionalItems<PostCommentUser>,
      pub timestamp: i64,
      pub is_video: bool,
      pub location: Location,
      pub user: MediumUser,
}
// Тип PostType
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PostType {
      /// Текстовое описание к посту.
      pub caption: String,

      /// Тип медиа-контента в посте (фотография, видео, карусель и т.д.).
      pub media_type: Option<MediaTypes>,

      /// URL медиа-контента (фотографии или видео) в посте.
      pub media_url: Option<String>,

      pub children: Option<Vec<MediaItem>>,

      /// URL миниатюры медиа-контента (для видео или карусели).
      pub thumbnail_url: Option<String>,

      /// Количество лайков, полученных постом.
      pub likes: AdditionalItems<MinimalUser>,

      /// Количество комментариев, оставленных к посту.
      pub comments: AdditionalItems<PostCommentUser>,

      /// Временная метка, указывающая на время публикации поста.
      pub timestamp: u64,

      /// Флаг, указывающий, является ли медиа-контент видео.
      pub is_video: bool,

      /// (объект): Информация о местоположении, где был опубликован пост, включая координаты и другие данные.
      pub location: Option<Location>,

      /// (объект): Информация о пользователе, который опубликовал пост, включая имя пользователя, идентификатор и другие данные.
      pub user: MediumUser,
}

// Перечисление MEDIA_TYPES
#[derive(Debug, Serialize, Deserialize)]
pub enum MediaTypes {
      // (изображение): Медиа-контент, представляющий собой статичное изображение. Обычно это формат JPEG или PNG.
      IMAGE,
      // (видео): Медиа-контент, представляющий собой видеофайл. В Instagram могут быть загружены видео с продолжительностью до 60 секунд.
      VIDEO,
      // (карусель): Карусельный пост, который содержит несколько изображений или видео. Пользователи могут пролистывать их горизонтально.
      CAROUSEL_ALBUM,
      // (IGTV): Медиа-контент, предназначенный для размещения в IGTV (Instagram TV). IGTV позволяет загружать более длительные видео, которые могут продолжаться до 60 минут.
      IGTV,
      // (история): Медиа-контент, используемый в историях (Stories). Истории в Instagram доступны в течение 24 часов и обычно представляют собой фотографии или короткие видеозаписи.
      STORY,
      // (альбом): Пост с несколькими изображениями или видео, который не является карусельным. В этом случае изображения или видео отображаются вертикально.
      ALBUM,
}
