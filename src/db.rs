use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use super::models::*;
use crate::activities;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_activity<'a>(conn: &'a PgConnection, name: &'a str) -> Activity {
    let new_activity = NewActivity {
        name,
    };
    diesel::insert_into(activities)
        .values(&new_activity)
        .get_result::<Activity>(&*conn)
        .expect("Error saving new actiitie.")
        // ;
    // new_activity
}
