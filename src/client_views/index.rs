use actix_web::{get, HttpResponse, HttpRequest};
use actix_files::{NamedFile};

#[get("/")]
async fn index_view(req: HttpRequest) -> HttpResponse {
    match NamedFile::open("static/index.html") {
        Ok(named_file) => named_file.into_response(&req),
        Err(error) => HttpResponse::Ok().body(
            format!("Error with index.html: {}", error.to_string())
        ),
    }
}
