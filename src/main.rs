mod api_views;
mod client_views;
mod config;
mod storage;
mod structs;
mod prelude {
    pub use super::{
        api_views::{api_views_config, views::*},
        client_views::{client_views_config::app_config, index},
        storage::db_io,
        structs::{activity::*, activitymanager::TaskManager},
    };
    pub use actix_files::NamedFile;
    pub use actix_web::{
        get, post,
        web::{self, Data, Json, Path, ServiceConfig},
        App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
    };
    pub use chrono::prelude::*;
    pub use serde::{
        ser::{SerializeStruct, Serializer},
        Deserialize, Serialize,
    };
    pub use std::{
        fs::File,
        io::prelude::*,
        path,
        sync::Mutex,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };
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
            .configure(api_views_config::app_config)
            .configure(app_config)
    })
    .bind((config.get_url().clone(), *config.get_port()))?
    .workers(4)
    .run()
    .await
}
