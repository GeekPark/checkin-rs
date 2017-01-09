extern crate csv;
pub extern crate rusqlite as sql;

use std::sync::Mutex;
use std::sync::Arc;
use self::sql::Result;

pub use self::sql::types::{ToSql, FromSql};

pub struct DB {
    pub conn: Arc<Mutex<sql::Connection>>,
}

use std;
impl DB {
    pub fn trace(sql: &str) {
        println!("SQL: {}", sql);
        let sql = sql.to_owned();
        std::thread::spawn(move || {
            use std::fs::OpenOptions;
            use std::io::prelude::*;

            let mut f =
                OpenOptions::new().create(true).append(true).open("operation_log.sql").unwrap();
            writeln!(f, "{}", sql).ok();
        });
    }
    pub fn connect() -> Self {
        let mut conn = sql::Connection::open("save.db").unwrap();
        conn.trace(Some(Self::trace));
        DB { conn: Arc::new(Mutex::new(conn)) }
    }

    pub fn insert<T>(&self, record: &T) -> bool
        where T: Record
    {
        let table_name = record.table_name();
        let fields = record.fields();
        let values = record.values();
        let sql_values = values.iter().map(|_| "?").collect::<Vec<&str>>().join(",");

        let sql = format!("INSERT INTO {table} ({fields}) VALUES ({values})",
                          table = table_name,
                          fields = fields.join(","),
                          values = sql_values);
        match self.conn.lock().unwrap().execute(&sql, &values) {
            Ok(_) => true,
            Err(e) => {
                println!("error! {:?}", e);
                false
            }
        }
    }

    pub fn create_table(&self, name: &str, schema: &str) -> Result<()> {
        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({});", name, schema);
        self.conn.lock().unwrap().execute_batch(&sql)
    }

    pub fn create_index(&self, table: &str, columns: &str) -> Result<()> {
        let index_name = columns.split_whitespace().collect::<Vec<&str>>().join("_");
        let sql = format!("CREATE INDEX IF NOT EXISTS {index_name} ON {table} ({columns});",
                          index_name = format!("{}_{}", table, index_name),
                          table = table,
                          columns = columns);
        self.conn.lock().unwrap().execute_batch(&sql)
    }

    pub fn search<T: Record>(&self, query: &str, args: &[&ToSql]) -> Vec<T> {
        let conn = self.conn.lock().unwrap();
        let mut sql = conn.prepare(query).unwrap();
        let mut v = Vec::new();
        for i in sql.query_map(args, |row| T::from_row(&row)).unwrap() {
            v.push(i.unwrap());
        }
        v
    }

    pub fn count(&self, query: &str, args: &[&ToSql]) -> Option<usize> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(query, args, |row| row.get::<i32, i32>(0) as usize).ok()
    }

    pub fn search_one<T: Record>(&self, query: &str, args: &[&ToSql]) -> Option<T> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(query, args, |row| T::from_row(&row)).ok()
    }

    pub fn update(&self,
                  table: &str,
                  query: &str,
                  query_params: &[&ToSql],
                  set: &str,
                  set_params: &[&ToSql]) {
        let conn = self.conn.lock().unwrap();
        let sql = format!("UPDATE {} SET {} WHERE {};", table, set, query);
        let mut stmt = conn.prepare(&sql).unwrap();
        let mut params = Vec::new();
        for param in set_params.iter().chain(query_params.iter()) {
            params.push(*param)
        }
        stmt.execute(params.as_slice()).unwrap();
    }
}

pub trait Record {
    fn from_row(vals: &sql::Row) -> Self;
    fn static_table_name() -> &'static str;
    fn static_fields() -> &'static [&'static str];
    fn values<'a>(&'a self) -> Vec<&'a ToSql>;
    fn fields<'a>(&'a self) -> &'a [&'a str] {
        Self::static_fields()
    }
    fn table_name<'a>(&'a self) -> &'a str {
        Self::static_table_name()
    }
}

lazy_static! {
    pub static ref _GLOBAL_DB: Arc<DB> = Arc::new(DB::connect());
}

use rocket::request::{FromRequest, Request};
use rocket::request;
pub struct DBI(pub Arc<DB>);

impl<'a, 'r> FromRequest<'a, 'r> for DBI {
    type Error = sql::Error;
    fn from_request(_: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        use rocket::outcome::Outcome;
        Outcome::Success(DBI(_GLOBAL_DB.clone()))
    }
}
