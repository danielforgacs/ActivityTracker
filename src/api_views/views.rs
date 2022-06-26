use actix_web::{App, HttpServer, HttpRequest, get};
use actix_web::web::{Path, Data};

use crate::{TaskManager, taskmanager_as_string};
use std::sync::Mutex;

#[get("start/{name}")]
pub async fn start_activity(name: Path<String>, req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.activate(&name);
    format!("activated task: {} Ok.", name)
}

#[get("stop/{name}")]
pub async fn stop_activity(name: Path<String>, req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.stop(&name);
    format!("stopped task: {} Ok.", name)
}

#[get("times")]
pub async fn times(req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let tm = data.lock().unwrap();
    println!("{}", taskmanager_as_string(&tm));
    tm.times()
}
