use crate::db::schema::logs;

#[derive(serde::Serialize, Queryable)]
pub struct Log {
    pub id: String,
    pub log_time: f64,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "logs"]
pub struct NewLog <'a> {
    pub body: &'a str,
}