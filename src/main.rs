mod api_views;
mod structs;

use actix_web::{App, HttpServer, HttpRequest, get};
use actix_web::web::{self, Path, Data};
use structs::taskmanager::TaskManager;
use api_views::views::*;
use std::sync::Mutex;
use clap;

const VERSION: &str = "0.1.0";
const ABOUT: &str = r#"
Web app to track time spent on activities.

endpoint:
    http://127.0.0.1:<PORT>/api/

default port is 8000.

views:
    activate/{name}     starts tracking a task. If the task doesn't exist it
                        will be created. All other tasks will be stopped,
                        only one task can be active at a time.
    times               returns times tracked for all tasks.
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
    println!("serving at: http://{}:{}/api/times", ADDRESS, port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(
                web::scope("/api")
                    .service(create_task)
                    .service(times)
            )
    })
    .bind((ADDRESS, port))?
    .workers(4)
    .run()
    .await
}
