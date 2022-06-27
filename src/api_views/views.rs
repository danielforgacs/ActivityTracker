use actix_web::{HttpRequest, get, HttpResponse};
use actix_web::web::{Path, Data};

use crate::{TaskManager, taskmanager_as_string};
use std::sync::Mutex;

#[get("{name}/start")]
pub async fn start_activity(name: Path<String>, req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.activate(&name);
    HttpResponse::Ok()
        .body(
            format!("activated task: {} Ok.", name)
        )
}

#[get("{name}/stop")]
pub async fn stop_activity(name: Path<String>, req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    HttpResponse::Ok()
        .body(
            if tm.stop(&name) {
                format!("stopped task: {} Ok.", name)
            } else {
                format!("error. can't stop task: {}.", name)
            }
        )
}

#[get("times")]
pub async fn times(req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let tm = data.lock().unwrap();
    println!("{}", taskmanager_as_string(&tm));
    HttpResponse::Ok()
        .body(
            tm.times()
        )
}
