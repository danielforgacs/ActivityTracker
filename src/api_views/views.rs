use actix_web::{App, HttpServer, HttpRequest, get};
use actix_web::web::{Path, Data};

use crate::{TaskManager, taskmanager_as_string};
use std::sync::Mutex;

#[get("activate/{name}")]
pub async fn create_task(name: Path<String>, req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.activate(&name);
    format!("task: {} Ok.", name)
}


#[get("times")]
pub async fn times(req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let tm = data.lock().unwrap();
    println!("{}", taskmanager_as_string(&tm));
    tm.times()
}
