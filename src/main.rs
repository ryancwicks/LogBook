#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::{content, Response}; 
use rocket::http::ContentType;
use std::io::Cursor;
use std::collections::HashMap;
use rocket::config::{Config, Environment};

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
embed_migrations!("./migrations");

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
fn index() -> content::Html<&'static str> {
    let index_string = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/static/index.html"));
    content::Html(index_string)
}

#[get("/static/logbook_builder.js")]
fn log_builder_js() -> content::JavaScript<&'static str> {
    let js_string = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/static/logbook_builder.js"));
    content::JavaScript(js_string)
}

#[get("/static/logbook_api.js")]
fn log_api_js() -> content::JavaScript<&'static str> {
    let js_string = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/static/logbook_api.js"));
    content::JavaScript(js_string)
}

#[get("/static/logbook_entry.js")]
fn log_entry_js() -> content::JavaScript<&'static str> {
    let js_string = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/static/logbook_entry.js"));
    content::JavaScript(js_string)
}

#[get("/static/favicon.ico")]
fn favicon() -> Response<'static> {
    let favicon_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),"/static/favicon.ico"));
    let mut response = Response::new();
    response.set_header(ContentType::Icon);
    response.set_sized_body(Cursor::new(favicon_bytes));
    response
}



pub fn rocket() -> rocket::Rocket {
    let mut databases = HashMap::new();
    let mut url = HashMap::new();
    url.insert("url", "log_database.sqlite3");
    databases.insert("sql_log", url);

    let mut config = Config::build(Environment::Production)
    .address("0.0.0.0")
    .port(5000)
    .secret_key("8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJ4=")
    .extra("databases", databases.clone())
    .finalize()
    .unwrap();

    let current_environment = Environment::active().unwrap();
    if current_environment.is_dev() {
        config = Config::build(Environment::Development)
        .address("localhost")
        .port(5000)
        .extra("databases", databases)
        .finalize()
        .unwrap();
    }

    rocket::custom(config)
    .attach (LogDbConn::fairing())
    .attach(rocket::fairing::AdHoc::on_attach("Database Migrations", run_db_migrations))
    .mount("/", routes![index, log_builder_js, log_api_js, log_entry_js, favicon]) 
    .mount("/api", routes![
        api::handle_new_log_item,
        api::get_all_records,])
}
fn main() {
    rocket().launch();
}


