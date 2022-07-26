mod api_views;
mod structs;
mod client_views;

use actix_web::{App, HttpServer};
use actix_web::web::{self, Data};
use structs::taskmanager::{TaskManager};
use api_views::views::*;
use std::sync::Mutex;
use client_views::index::*;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "8000";

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let matches = clap::Command::new("timetracker")
        .version(env!("CARGO_PKG_VERSION"))
        .about(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md")))
        .arg(
            clap::Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .value_parser(clap::value_parser!(u16).range(3000..))
                .default_value(PORT)
                .help("Set the localhost port to serve.")
        )
        .get_matches();
    let port: u16 = *matches.get_one("port").unwrap();
    let data = Data::new(Mutex::new(TaskManager::new()));
    println!("web: http://{}:{}/", ADDRESS, port);
    println!("api: http://{}:{}/api/times", ADDRESS, port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(index_view)
            .service(
                web::scope("/api")
                    .service(start)
                    .service(stop)
                    .service(times)
                    .service(pretty)
            )
    })
    .bind((ADDRESS, port))?
    .workers(4)
    .run()
    .await
}
