use crate::db::schema::logs;
use crate::db::schema::logs::dsl::{logs as all_logs};

use diesel::{self, QueryResult, prelude::*};
use chrono::NaiveDateTime;

#[table_name = "logs"]
#[derive(serde::Serialize, Queryable,Insertable,Debug,Clone)]
pub struct Log {
    pub id: Option<i32>,
    pub log_time: NaiveDateTime,
    pub body: String,
}

impl Log {
    /// Returns all rows in the table sorted by time.
    pub fn all(conn: &SqliteConnection) -> QueryResult<Vec<Log>> {
        all_logs.order(logs::id.desc()).load::<Log>(conn)    
    }

    /// Returns the inserted row.
    pub fn insert(log_string: String, conn: &SqliteConnection) -> QueryResult<usize> {
        let log_time = chrono::Utc::now().naive_utc();
        let l = Log{ id: None, log_time: log_time, body: log_string};
        diesel::insert_into(logs::table)
                    .values(&l)
                    .execute(conn)        
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> usize {
        diesel::delete(all_logs).execute(conn)?;
    }
}