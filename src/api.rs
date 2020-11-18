use rocket_contrib::json::Json;
use rocket::response::content;

use crate::log_book_item;

#[get("/test")]
pub fn test_api() -> &'static str {
    "In API."
}

#[post("/add_item", data = "<log_item>")]
pub fn handle_new_log_item(log_item: Option<Json<log_book_item::LogItem>>, conn: LogDbConn) -> content::Json<&'static str> {
    
    content::Json("{ 
        'success': False, 
        'message': 'Add new log item not implemented.' 
    }")
}

