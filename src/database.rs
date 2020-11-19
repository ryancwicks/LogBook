use rocket_contrib::databases::diesel;

use crate::db::models;
use crate::db::schema;

#[database("sql_log")]
pub struct LogDbConn(diesel::SqliteConnection);