use crate::prelude::*;

#[derive(Deserialize)]
pub struct StartJson {
    name: String,
}

#[derive(Deserialize)]
pub struct ActivityDate {
    date: String,
}

pub async fn start(req: HttpRequest, form: web::Json<StartJson>) -> HttpResponse {
    let name = form.into_inner().name;
    let data = req.app_data::<Data<Mutex<ActivityManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.start_activity(name.as_str());
    HttpResponse::Ok().body(format!("activated task: {} Ok.", name))
}

pub async fn stop(req: HttpRequest) -> impl Responder {
    let data = req.app_data::<Data<Mutex<ActivityManager>>>().unwrap();
    let mut tm = data.lock().unwrap();
    tm.stop();
    "ok"
}

pub async fn get_activities_per_date(date: web::Json<ActivityDate>, req: HttpRequest) -> HttpResponse {
    log::info!("get_activities_per_date()");
    let data = req.app_data::<Data<Mutex<ActivityManager>>>().unwrap();
    let date = date.into_inner().date;
    let tm = data.lock().unwrap().clone();
    HttpResponse::Ok().json(tm.get_activities_by_date(date))
}

pub async fn pretty(req: HttpRequest) -> String {
    let data = req.app_data::<Data<Mutex<ActivityManager>>>().unwrap();
    let tm = data.lock().unwrap();
    tm.pretty()
}
