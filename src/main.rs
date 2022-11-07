mod api_views;
mod client_views;
mod config;
mod storage;
mod structs;
mod utils;
mod prelude {
    pub use super::utils::*;
    pub use super::{
        api_views::{api_views_config, views::*},
        client_views::{client_views_config::app_config, index},
        storage::db_io,
        structs::{activity::*, activitymanager::ActivityManager},
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
        collections::HashMap,
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
    env_logger::init();
    log::info!("Got the config.");
    println!(
        "web: http://{}:{}\napi: http://{}:{}/api/times",
        config.get_url(),
        config.get_port(),
        config.get_url(),
        config.get_port()
    );

    let data = Data::new(Mutex::new(ActivityManager::new(
        config.get_dbpath().clone(),
    )));

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


#[cfg(test)]
mod tests {
    use super::*;
    use super::structs::activitymanager::ActivityManagerSerial;
    use actix_web::{test, App};
    use actix_web::http::header;

    #[actix_web::test]
    async fn test_create_activity() {
        let mut test_db = path::PathBuf::new();
        test_db.push("test_db.json");
        let data = Data::new(Mutex::new(ActivityManager::new(test_db)));
        let app = test::init_service(
            App::new()
                .app_data(Data::clone(&data))
                .configure(api_views_config::app_config)
                .configure(app_config)
        ).await;
        let payload = format!(r#"{{"date":"{}"}}"#, Utc::now().date_naive());
        let req = test::TestRequest::post()
            .uri("/api/activities")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(payload)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let result: ActivityManagerSerial = test::read_body_json(resp).await;
    }
}
