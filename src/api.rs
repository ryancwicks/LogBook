use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use std::fmt::Write;

use crate::LogDbConn;
use crate::db::models::Log;

#[derive(Serialize, Deserialize)]
pub struct LogItem {
    log: String,
}

#[derive(Serialize, Deserialize)]
pub struct StandardResponse {
    pub success: bool,
    pub message: String,
    pub records: Option<Vec<Log>>,
}

#[post("/add_item", format = "json", data = "<log_item>")]
pub fn handle_new_log_item(log_item: Json<LogItem>, conn: LogDbConn) -> Json<StandardResponse> {

    let log_data = log_item.0.log;

    if log_data.is_empty() {
        Json(StandardResponse {
            success: false,
            message: "Can't insert empty log.".into(),
            records: None
        })
    } else if let Err(e) = Log::insert(log_data.clone(), &conn) {
        let mut message: String = "".to_string();
        match writeln!(message, "DB insertion error: {}", e) {
            Ok(_) => Json(StandardResponse {
                    success: false,
                    message: message,
                    records: None
                    }),
            Err(_) => Json(StandardResponse {
                    success: false,
                    message: "Things have gone very badly wrong.".into(),
                    records: None
                })
        }
        
    } else {
        let log_time = chrono::Utc::now().naive_utc();
        let l = Log{ id: None, log_time: log_time, body: log_data}; //bit of a hack to send the latest add without reloading the entire database.
        let mut records: Vec<Log> = Vec::new();
        records.push(l);
        Json(StandardResponse {
            success: true,
            message: "".into(),
            records: Some(records),
        })
    }
}

#[get("/all")]
pub fn get_all_records(conn: LogDbConn) ->Json<StandardResponse> {
    let records = match Log::all(&conn) {
        Ok(val) => val,
        Err(_e) => {
            return Json(StandardResponse {
                success: false,
                message: "Can't read logs from the database.".into(),
                records: None
            })
        }
    };
    Json(StandardResponse {
        success: true,
        message: "".into(),
        records: Some(records)
    })
}

