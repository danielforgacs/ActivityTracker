mod api_views;
mod structs;

use actix_web::{App, HttpServer, HttpRequest, get};
use actix_web::web::{Path, Data};
use structs::taskmanager::TaskManager;
use api_views::views::create_task;
use std::sync::Mutex;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8000;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let data = Data::new(Mutex::new(TaskManager::new()));
    println!("Starting: http://{}:{}", ADDRESS, PORT);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(create_task)
    })
    .bind((ADDRESS, PORT))?
    .workers(4)
    .run()
    .await
}
