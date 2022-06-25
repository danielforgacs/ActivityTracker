use actix_web::{App, HttpServer, HttpRequest, get};
use actix_web::web::{Path, Data};

use crate::TaskManager;
use std::sync::Mutex;

#[get("create_task/{name}")]
pub async fn create_task(name: Path<String>, req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.activate(&name);
    println!("tm: {:#?}", &tm);
    "Ok".to_string()
}
