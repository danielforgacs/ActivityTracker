mod api_views;
mod client_views;
mod config;
mod structs;
mod prelude {
    pub use std::fs::File;
    pub use std::path;
    pub use std::io::prelude::*;
    pub use std::sync::Mutex;
    pub use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub use actix_files::NamedFile;
    pub use actix_web::web::{self, Data};
    pub use actix_web::web::{Json, Path};
    pub use actix_web::{get, HttpRequest, HttpResponse};
    pub use actix_web::{post, Responder, Result};
    pub use actix_web::{App, HttpServer};
    pub use chrono::prelude::*;
    pub use serde::ser::{SerializeStruct, Serializer};
    pub use serde::{Deserialize, Serialize};

    pub use super::api_views::views::*;
    pub use super::client_views::index::*;
    pub use super::structs::activity::*;
    pub use super::structs::activitymanager::TaskManager;

    pub const DAY_LENGTH_SECS: u64 = 7 * 60 * 60 + 30 * 60;
    pub type SecType = u64;
}
use prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match config::get_congig() {
        Ok(config) => config,
        Err(msg) => {
            println!("{}", msg);
            return Ok(());
        }
    };
    println!("web: http://{}:{}/", config.url, config.port);
    println!("api: http://{}:{}/api/times", config.url, config.port);

    let data = Data::new(Mutex::new(TaskManager::new(config.dbpath)));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(index_view)
            .service(
                web::scope("/api")
                    .service(start)
                    .service(stop)
                    .service(times)
                    .service(pretty),
            )
    })
    .bind((config.url, config.port))?
    .workers(4)
    .run()
    .await
}
