use actix_web::{get, HttpResponse, HttpRequest};
use actix_files::{NamedFile};

#[get("/")]
async fn index_view(req: HttpRequest) -> HttpResponse {
    let exe_path = std::env::current_exe().unwrap();
    let current_dir = exe_path.parent().unwrap();
    let index_path = format!("{}/{}", current_dir.display(), "static/index.html");
    let index_path_ref = index_path.clone();
    match NamedFile::open(index_path) {
        Ok(named_file) => named_file.into_response(&req),
        Err(error) => HttpResponse::Ok().body(
            format!("Error with index.html: {}. Expected path: {}.", error.to_string(), index_path_ref)
        ),
    }
}
