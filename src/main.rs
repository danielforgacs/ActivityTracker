mod structs;

use actix_web::{App, HttpServer, get};
use actix_web::web::{Path, Data};
use structs::taskmanager::TaskManager;
use std::sync::Mutex;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8000;

#[get("create_task/{name}")]
async fn create_task(name: Path<String>) -> String {
    "Ok".to_string()
}

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
