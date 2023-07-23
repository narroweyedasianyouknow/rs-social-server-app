pub mod auth;
pub mod db;
use actix_web::rt::time::interval;
use actix_web::{web, HttpResponse, Responder};
use actix_web::{
      web::{post, resource, Data},
      App, HttpServer,
};
use actix_web_lab::extract::Path;
use actix_web_lab::sse::{self, ChannelStream, Sse};
use db::{create_unique_index, get_db};
use dotenv::dotenv;
use futures_util::future;
use parking_lot::Mutex;
use routes::posts::like_post;
use routes::{
      authorization::{login, registration},
      posts::create_post,
      uploader::image_upload,
};
use std::{sync::Arc, time::Duration};
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
            pub mod like_post;
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

pub struct AppState {
      broadcaster: Arc<Broadcaster>,
}
#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
      clients: Vec<sse::Sender>,
}

pub struct Broadcaster {
      inner: Mutex<BroadcasterInner>,
}

impl Broadcaster {
      /// Constructs new broadcaster and spawns ping loop.
      pub fn create() -> Arc<Self> {
            let this = Arc::new(Broadcaster {
                  inner: Mutex::new(BroadcasterInner::default()),
            });
            Broadcaster::spawn_ping(Arc::clone(&this));
            // println!("created success");

            this
      }

      /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast list if not.
      fn spawn_ping(this: Arc<Self>) {
            actix_web::rt::spawn(async move {
                  let mut interval = interval(Duration::from_secs(10));

                  loop {
                        interval.tick().await;
                        this.remove_stale_clients().await;
                  }
            });
      }

      /// Removes all non-responsive clients from broadcast list.
      async fn remove_stale_clients(&self) {
            let clients = self.inner.lock().clients.clone();
            println!("active client {:?}", clients);

            let mut ok_clients = Vec::new();

            println!("okay active client {:?}", ok_clients);

            for client in clients {
                  if client
                        .send(sse::Event::Comment("ping".into()))
                        .await
                        .is_ok()
                  {
                        ok_clients.push(client.clone());
                  }
            }

            self.inner.lock().clients = ok_clients;
      }

      /// Registers client with broadcaster, returning an SSE response body.
      pub async fn new_client(&self) -> Sse<ChannelStream> {
            println!("starting creation");
            let (tx, rx) = sse::channel(10);

            tx.send(sse::Data::new("connected")).await.unwrap();
            println!("creating new clients success {:?}", tx);
            self.inner.lock().clients.push(tx);
            rx
      }

      /// Broadcasts `msg` to all clients.
      pub async fn broadcast(&self, msg: &str) {
            let clients = self.inner.lock().clients.clone();

            let send_futures = clients
                  .iter()
                  .map(|client| client.send(sse::Data::new(msg)));

            // try to send to all clients, ignoring failures
            // disconnected clients will get swept up by `remove_stale_clients`
            let _ = future::join_all(send_futures).await;
      }
}

pub async fn sse_client(state: web::Data<AppState>) -> impl Responder {
      state.broadcaster.new_client().await
}

pub async fn broadcast_msg(
      state: web::Data<AppState>,
      Path((msg,)): Path<(String,)>,
) -> impl Responder {
      state.broadcaster.broadcast(&msg).await;
      HttpResponse::Ok().body("msg sent")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
      dotenv().ok();
      // let broadcaster = Broadcaster::create();

      let url = &env::var("BACKEND_URI").expect("BACKEND_URI not set");
      // Подключение к базе данных
      let db = get_db().await;
      let users_collection: mongodb::Collection<StoredUserType> =
            db.collection(COLLECTION_NAMES.users.as_str());
      let _ = create_unique_index(&users_collection).await;
      HttpServer::new(move || {
            App::new()
                  .route("/registration", post().to(registration::main))
                  .route("/login", post().to(login::main))
                  .service(resource("/upload").route(post().to(image_upload::upload_file)))
                  // .app_data(web::Data::new(AppState {
                  //     broadcaster: Arc::clone(&broadcaster),
                  // }))
                  // .route("/events{_:/?}", web::get().to(sse_client))
                  // .route("/events/{msg}", web::get().to(broadcast_msg))
                  .route("/post", post().to(create_post::main))
                  .route("/like/{post_id}", post().to(like_post::main))
                  .app_data(Data::new(db.clone()))
      })
      .bind(url)?
      .run()
      .await
}
