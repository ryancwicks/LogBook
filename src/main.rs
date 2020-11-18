#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::{NamedFile};
use rocket::response::status::{NotFound};
use rocket_contrib::databases::sqlite;

use std::path::Path;

mod api;
mod log_book_item;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[database("sqlite_logs")]
struct LogDbConn(diesel::SqliteConnection);

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

fn main() {
    rocket::ignite().attach (LogDbConn::fairing())
                    .mount("/", routes![index, stylesheet])
                    .mount("/api", routes![
                        api::test_api,
                        api::handle_new_log_item])
                    .launch();
}
