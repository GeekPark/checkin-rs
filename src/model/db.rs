extern crate csv;
pub extern crate rusqlite as sql;

use std::sync::Mutex;
use std::sync::Arc;
use self::sql::Result;

pub use self::sql::types::ToSql;

pub struct DB {
    conn: Arc<Mutex<sql::Connection>>,
}

impl DB {
    pub fn connect() -> Self {
        let conn = sql::Connection::open("save.db").unwrap();
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
            Err(_) => false,
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
}

pub trait Record {
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
