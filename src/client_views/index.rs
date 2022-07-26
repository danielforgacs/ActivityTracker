use actix_web::{get, HttpResponse};

const INDEX_HTML: &'static str = include_str!("index.html");

#[get("/")]
async fn index_view() -> HttpResponse {
    HttpResponse::Ok()
        .body(INDEX_HTML)
}
