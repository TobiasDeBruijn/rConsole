use rusqlite::Connection;
use std::path::PathBuf;

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Database, String> {
        let conn_wrapped = Connection::open(path.as_path());
        if conn_wrapped.is_err() {
            return Err(conn_wrapped.err().unwrap().to_string());
        }

        let conn = conn_wrapped.unwrap();

        Ok(Database {
            connection: conn
        })
    }
}

