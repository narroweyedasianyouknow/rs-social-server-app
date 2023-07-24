use actix_web::{body::BoxBody, cookie::Cookie, http::StatusCode, HttpResponse};
use bson::Document;

pub fn request_response<'a>(
      error: bool,
      message: Option<String>,
      status: Option<u16>,
      cookie: Option<Cookie<'a>>,
) -> HttpResponse<BoxBody> {
      let mut document = Document::new();
      document.insert("error", error);
      if message.is_some() {
            document.insert("message", message);
      }

      let mut status = match status {
            Some(code) => HttpResponse::build(StatusCode::from_u16(code).unwrap()),
            None => HttpResponse::BadRequest(),
      };
      if cookie.is_some() {
            status.cookie(cookie.unwrap());
      }

      return status.json(document);
}
