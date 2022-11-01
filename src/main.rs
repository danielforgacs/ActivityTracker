mod api_views;
mod client_views;
mod config;
mod structs;

use actix_web::web::{self, Data};
use actix_web::{App, HttpServer};
use api_views::views::*;
use client_views::index::*;
use std::sync::Mutex;
use structs::taskmanager::TaskManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_congig();
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
