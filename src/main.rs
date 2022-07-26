mod api_views;
mod structs;
mod client_views;

use actix_web::{App, HttpServer};
use actix_web::web::{self, Data};
use structs::taskmanager::{TaskManager};
use api_views::views::*;
use std::sync::Mutex;
use client_views::index::*;

const VERSION: &str = "2022.7.10";
const ABOUT: &str = r#"
Web app to track time spent on activities.

base endpoint:
    http://127.0.0.1:<PORT>/api/

default port is 8000.

api endpoints:
    start/{name}        starts tracking an activity. If it doesn't exist it
                        will be created. All other activities will be stopped,
                        only one activity can be active at a time.
    stop                stops any activity.
    times               returns the taskmanager as json.
    pretty              return the taskmanager as formatted string.
"#;
const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "8000";

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let matches = clap::Command::new("timetracker")
        .version(VERSION)
        .about(ABOUT)
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
