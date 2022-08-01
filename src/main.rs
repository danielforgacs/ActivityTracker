mod api_views;
mod structs;
mod client_views;
mod config;

use actix_web::{App, HttpServer};
use actix_web::web::{self, Data};
use structs::taskmanager::{TaskManager};
use api_views::views::*;
use std::sync::Mutex;
use client_views::index::*;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let config = config::get_congig();
    let data = Data::new(Mutex::new(TaskManager::new()));
    println!("web: http://{}:{}/", config::ADDRESS, config.port);
    println!("api: http://{}:{}/api/times", config::ADDRESS, config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(index_view)
            .service(
                web::scope("/api")
                    .service(start)
                    .service(stop)
                    .service(times)
                    .service(pretty)
            )
    })
        .bind((config::ADDRESS, config.port))?
        .workers(4)
        .run()
        .await
}
