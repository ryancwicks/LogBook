use super::schema::logs;

#[derive(Serialize, Queryable)]
pub struct Log {
    pub id: String,
    pub log_time: float64,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "logs"]
pub struct NewLog <'a> {
    pub id: &'a str,
    pub log_time: &'a str,
    pub body: &'a str,
}