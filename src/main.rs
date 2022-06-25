mod structs;
mod views;

use std::sync::Mutex;
use actix_web::{App, HttpServer, web::Data};

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8000;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let task_manager = Data::new(

    )
    println!("serving http://{}:{}", ADDRESS, PORT);
    HttpServer::new(|| {
        App::new()
    })
    .bind((ADDRESS, PORT))?
    .workers(4)
    .run()
    .await
}
