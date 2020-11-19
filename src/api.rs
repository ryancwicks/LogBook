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
    time: u64,
    description: String,
}

#[get("/test")]
pub fn test_api() -> &'static str {
    "In API."
}

#[post("/add_item", data = "<log_item>")]
pub fn handle_new_log_item(log_item: Option<Json<LogItem>>, conn: LogDbConn) -> Json<StandardResponse> {

    Json(StandardResponse {
        success: true,
        message: "Not implemented.".into(),
    })
}

