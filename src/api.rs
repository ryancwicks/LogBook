use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use std::fmt::Write;

use crate::LogDbConn;
use crate::db::models::Log;

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

    if log_data.is_empty() {
        let message = "Can't insert empty log.";
        Json(StandardResponse {
            success: false,
            message: message.into()
        })
    } else if let Err(e) = Log::insert(log_data, &conn) {
        let mut message: String = "".to_string();
        match writeln!(message, "DB insertion error: {}", e) {
            Ok(_) => Json(StandardResponse {
                    success: false,
                    message: message
                    }),
            Err(_) => Json(StandardResponse {
                    success: false,
                    message: "Things have gone very badly wrong.".into()
                })
        }
        
    } else {
        Json(StandardResponse {
            success: true,
            message: "".into(),
        })
    }
}


