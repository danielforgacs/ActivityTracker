use crate::TaskManager;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpRequest, HttpResponse, Responder, Result};
use std::sync::Mutex;

#[post("start/{name}")]
pub async fn start(name: Path<String>, req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.start(&name);
    HttpResponse::Ok().body(format!("activated task: {} Ok.", name))
}

#[post("stop")]
pub async fn stop(req: HttpRequest) -> impl Responder {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.stop();
    "ok"
}

#[get("times")]
pub async fn times(req: HttpRequest) -> Result<impl Responder> {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let tm = data.lock().unwrap().clone();
    Ok(Json(tm))
}

#[get("pretty")]
pub async fn pretty(req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let tm = data.lock().unwrap();
    println!("{}", &tm.times());
    tm.times()
}
