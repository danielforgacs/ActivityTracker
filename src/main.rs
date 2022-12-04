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
    pub use actix_files::{self, NamedFile};
    pub use actix_web::{
        get, post,
        web::{self, Data, Json, Path, ServiceConfig},
        App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
    };
    pub use chrono::prelude::*;
    pub use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
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

    // create certificate:
    // Add a passphrase or it won't work. The file will be empty.
    // openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
    let mut ssl_builder =
        match SslAcceptor::mozilla_intermediate(SslMethod::tls()) {
            Ok(builder) => builder,
            Err(_) => {
                println!("Can not create SSL BUilder.");
                return Ok(());
            }
        };
    match ssl_builder.set_private_key_file("key.pem", SslFiletype::PEM) {
        Ok(()) => {}
        Err(_) => {
            println!("Can not set SSL private key file.");
            return Ok(());
        }
    };
    match ssl_builder.set_certificate_chain_file("cert.pem") {
        Ok(()) => {}
        Err(_) => {
            println!("Can not set SSL certificate chain file.");
            return Ok(());
        }
    };

    println!(
        "web: http://{}\napi: http://{}/api/times",
        config.get_url_w_port(),
        config.get_url_w_port(),
    );

    let data = Data::new(Mutex::new(ActivityManager::new(
        config.get_dbpath().clone(),
    )));

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(
                actix_files::Files::new("/static", "static")
                    .show_files_listing(),
            )
            .configure(api_views_config::app_config)
            .configure(app_config)
    })
    .bind_openssl(config.get_url_w_port(), ssl_builder)?
    .workers(4)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header;
    use actix_web::{test, App};

    #[test]
    async fn bad_test_f_github_actions() {
        assert!(false);
    }

    #[actix_web::test]
    async fn test_create_activity() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Response {
            date: String,
            activities: Vec<ActivitySerial>,
        }
        let mut test_db = path::PathBuf::new();
        test_db.push("test_db.json");
        std::fs::write(&test_db, b"[]").unwrap();
        let data = Data::new(Mutex::new(ActivityManager::new(test_db.clone())));
        let app = test::init_service(
            App::new()
                .app_data(Data::clone(&data))
                .configure(api_views_config::app_config)
                .configure(app_config),
        )
        .await;
        {
            let payload =
                format!(r#"{{"date":"{}"}}"#, Utc::now().date_naive());
            let req = test::TestRequest::post()
                .uri("/api/activities")
                .insert_header((header::CONTENT_TYPE, "application/json"))
                .set_payload(payload)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
            let result: Response = test::read_body_json(resp).await;
            assert_eq!(
                result,
                Response {
                    date: Utc::now().date_naive().to_string(),
                    activities: vec![],
                }
            );
        }
        {
            let req = test::TestRequest::post()
                .uri("/api/start")
                .insert_header((header::CONTENT_TYPE, "application/json"))
                .set_payload(r#"{"name":"test_name_01"}"#.as_bytes())
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
            #[derive(Debug, Deserialize, PartialEq)]
            struct ResponseActivity {
                name: String,
            }
            let result: ResponseActivity = test::read_body_json(resp).await;
            assert_eq!(
                result,
                ResponseActivity {
                    name: "test_name_01".to_string(),
                }
            );
        }
        std::fs::remove_file(test_db).unwrap();
    }
}
