#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::{content, Response}; //{NamedFile};
use rocket::http::ContentType;
use std::io::Cursor;
//use rocket_contrib::serve::StaticFiles;
//use std::path::Path;

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
    //content::Content(ContentType::Icon, favicon_string);
    let mut response = Response::new();
    response.set_header(ContentType::Icon);
    response.set_sized_body(Cursor::new(favicon_bytes));
    response
}

//#[get("/")]
//fn index() -> Result<NamedFile, NotFound<String>> {
//    let path = Path::new("static/index.html");
//    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
//}

//#[get("/static/<filename>")]
//fn stylesheet(filename: &RawStr) -> Result<NamedFile, NotFound<String>> {
//    let full_filename = format!("static/{filename}", filename=filename);
//    let path = Path::new(&full_filename);
//    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
//}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().attach (LogDbConn::fairing())
    .attach(rocket::fairing::AdHoc::on_attach("Database Migrations", run_db_migrations))
    .mount("/", routes![index, log_builder_js, log_api_js, log_entry_js, favicon])
    //.mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"),"/static"))) 
    .mount("/api", routes![
        api::handle_new_log_item,
        api::get_all_records,])
}
fn main() {
    rocket().launch();
}


