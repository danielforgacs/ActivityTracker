use actix_web::{App, HttpServer};

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8000;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Starting: http://{}:{}", ADDRESS, PORT);
    HttpServer::new(|| {
        App::new()
    })
    .bind((ADDRESS, PORT))?
    .workers(4)
    .run()
    .await
}
