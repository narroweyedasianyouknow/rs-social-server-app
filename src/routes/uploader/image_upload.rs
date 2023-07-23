use image::DynamicImage;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use actix_multipart::Multipart;
use actix_web::http::header::CONTENT_LENGTH;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::TryStreamExt;
use uuid::Uuid;

pub async fn upload_file(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
      let uploads_folder = "./uploads/images/";
      let thumbnails_folder = "./uploads/thumbnails/";
      let max_file_size = 10_000;
      let max_file_count = 10;
      let legal_filetypes = [mime::IMAGE_PNG, mime::IMAGE_JPEG, mime::IMAGE_GIF];

      let content_length = match req.headers().get(CONTENT_LENGTH) {
            Some(hv) => hv.to_str().unwrap_or("0").parse().unwrap(),
            None => 0,
      };

      if content_length == 0 || content_length > max_file_size {
            HttpResponse::BadRequest().body("hello");
      }
      let mut current_count = 0;

      loop {
            if current_count >= max_file_count {
                  break;
            }

            if let Ok(Some(mut field)) = payload.try_next().await {
                  if field.name() != "upload" {
                        continue;
                  }
                  let filetype = field.content_type();
                  if filetype.is_none() {
                        continue;
                  }
                  if !legal_filetypes.contains(filetype.unwrap()) {
                        continue;
                  }

                  let destination = format!(
                        "{}{}-{}",
                        uploads_folder,
                        Uuid::new_v4(),
                        field.content_disposition().get_filename().unwrap()
                  );

                  let mut saved_file = fs::File::create(&destination).await.unwrap();
                  while let Ok(Some(chunk)) = field.try_next().await {
                        let _ = saved_file.write_all(&chunk).await.unwrap();
                  }

                  web::block(move || async move {
                        let updated_img: DynamicImage = image::open(&destination).unwrap();
                        let _ = fs::remove_file(&destination).await.unwrap();
                        let file_name = Uuid::new_v4();
                        updated_img
                              .thumbnail(100, 100)
                              .save(format!("{}{}.{}", thumbnails_folder, file_name, "jpeg"))
                              .unwrap();
                        updated_img
                              .save(format!("{}{}.{}", uploads_folder, file_name, "jpeg"))
                              .unwrap();
                  })
                  .await
                  .unwrap()
                  .await;
            } else {
                  break;
            }

            current_count += 1;
      }
      HttpResponse::Ok().body("Hello")
}
