#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::{NamedFile};
use rocket::response::status::{NotFound};
use std::path::Path;

mod api;
mod db;


#[cfg(test)] mod tests;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;

/// Database definition
#[database("sql_log")]
pub struct LogDbConn(diesel::SqliteConnection);

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

fn run_db_migrations(rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
    let conn = LogDbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

#[get("/")]
fn index() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/index.html");
    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}

#[get("/style.css")]
fn stylesheet() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/style.css");
    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().attach (LogDbConn::fairing())
    .attach(rocket::fairing::AdHoc::on_attach("Database Migrations", run_db_migrations))
    .mount("/", routes![index, stylesheet])
    .mount("/api", routes![
        api::test_api,
        api::handle_new_log_item])
}
fn main() {
    rocket().launch();
}


