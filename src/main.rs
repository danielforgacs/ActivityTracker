mod api_views;
mod structs;
mod client_views;
mod config;
mod db;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use actix_web::web::{self, Data};
use structs::taskmanager::{TaskManager};
use api_views::views::*;
use std::sync::Mutex;
use client_views::index::*;
use schema::activities::dsl::*;
use diesel::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let config = config::get_congig();
    println!("web: http://{}:{}/", config.url, config.port);
    println!("api: http://{}:{}/api/times", config.url, config.port);

    let data = Data::new(
        Mutex::new(
            TaskManager::new()
        )
    );

    let db_conn = db::establish_connection();
    let result = activities
        .load::<models::Activity>(&db_conn);
    dbg!(&result);

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
        .bind((config.url, config.port))?
        .workers(4)
        .run()
        .await
}
