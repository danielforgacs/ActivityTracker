use crate::prelude::*;

pub async fn start(name: Path<String>, req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.start_activity(&name);
    HttpResponse::Ok().body(format!("activated task: {} Ok.", name))
}

pub async fn stop(req: HttpRequest) -> impl Responder {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.stop();
    "ok"
}

pub async fn times(req: HttpRequest) -> Result<impl Responder> {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let tm = data.lock().unwrap().clone();
    Ok(Json(tm.today_times()))
}

pub async fn pretty(req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    let tm = data.lock().unwrap();
    tm.pretty()
}
