use rocket_contrib::databases::diesel;

pub mod models;
pub mod schema;

#[database("sql_log")]
pub struct LogDbConn(diesel::SqliteConnection);