use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

use crate::database::LogDbConn;

#[derive(Serialize, Deserialize)]
pub struct StandardResponse {
    pub success: bool,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogItem {
    log: String,
}

#[get("/test")]
pub fn test_api() -> &'static str {
    "In API."
}

#[post("/add_item", format = "json", data = "<log_item>")]
pub fn handle_new_log_item(log_item: Json<LogItem>, conn: LogDbConn) -> Json<StandardResponse> {

    let log_data = log_item.0.log;



    Json(StandardResponse {
        success: true,
        message: "".into(),
    })
}

