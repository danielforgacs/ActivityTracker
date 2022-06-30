use crate::{TaskManager};
use actix_web::{HttpRequest, get, HttpResponse, Result, Responder};
use actix_web::web::{Path, Data, Json};
use std::sync::Mutex;

#[get("start/{name}")]
pub async fn start_activity(name: Path<String>, req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.activate(&name);
    HttpResponse::Ok()
        .body(
            format!("activated task: {} Ok.", name)
        )
}

#[get("stop")]
pub async fn stop_all(req: HttpRequest) -> impl Responder {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.stop_all();
    "ok"
}

#[get("times")]
pub async fn times(req: HttpRequest) -> Result<impl Responder> {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let tm = data.lock().unwrap().clone();
    Ok(Json(tm))
}

#[get("pretty")]
pub async fn pretty_print(req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let tm = data.lock().unwrap();
    println!("{}", &tm.times());
    tm.times()
}
