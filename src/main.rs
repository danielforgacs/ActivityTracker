mod api_views;
mod client_views;
mod config;
mod storage;
mod structs;
mod prelude {
    pub use std::fs::File;
    pub use std::io::prelude::*;
    pub use std::path;
    pub use std::sync::Mutex;
    pub use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub use actix_files::NamedFile;
    pub use actix_web::web::{self, Data, ServiceConfig};
    pub use actix_web::web::{Json, Path};
    pub use actix_web::{get, HttpRequest, HttpResponse};
    pub use actix_web::{post, Responder, Result};
    pub use actix_web::{App, HttpServer};
    pub use chrono::prelude::*;
    pub use serde::ser::{SerializeStruct, Serializer};
    pub use serde::{Deserialize, Serialize};

    pub use super::api_views::app_config;
    pub use super::api_views::views::*;
    pub use super::client_views::app_config::app_config;
    pub use super::client_views::index;
    pub use super::storage::db_io;
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
    println!("web: http://{}:{}/", config.get_url(), config.get_port());
    println!(
        "api: http://{}:{}/api/times",
        config.get_url(),
        config.get_port()
    );

    let data = Data::new(Mutex::new(TaskManager::new(config.get_dbpath().clone())));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .configure(app_config::app_config)
            .configure(app_config)
    })
    .bind((config.get_url().clone(), *config.get_port()))?
    .workers(4)
    .run()
    .await
}
