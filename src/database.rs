use std::{collections::HashMap, path::PathBuf, sync::{Arc, Mutex}};

use crate::Result;

use fs_walk::WalkOptions;
use rusqlite::Connection;

pub type Conn = Arc<Mutex<Connection>>;



pub fn find_files(dir: &str) -> HashMap<String, String> {
    let mut r = HashMap::new();
    let w = WalkOptions::new()
        .files()
        .extension("sql")
        .walk(dir);
    for path in w.flatten() {
        let content = std::fs::read_to_string(&path).unwrap();
        r.insert(path.to_string_lossy().to_string(), content); //(content);
    }
    return r;
}



#[derive(Debug, Clone)]
pub struct Database {
    pub conn: Conn,
}

impl Database {
    pub fn new(path: PathBuf, db_name: &str) -> Result<Database> {
        if std::fs::metadata(&path).is_err() {
            std::fs::create_dir_all(&path).unwrap();
        }
        let conn = Connection::open(path.join(db_name))?;
        Ok(Database { conn: Arc::new(Mutex::new(conn)) })
    }

    pub fn new_memory() -> Result<Database> {
        let conn = Connection::open_in_memory()?;
        Ok(Database { conn: Arc::new(Mutex::new(conn)) })
    }

    pub fn migrate(&self, directory: &str) -> Result<()> {
        let files = find_files(directory);
        let conn = self.conn.lock().unwrap();
        for (_name, data) in files {
            conn.execute_batch(&data)?;
        }
        Ok(())
    }

    pub fn close(self) {
        drop(self.conn);
    }

    pub fn conn(&self) -> Conn {
        self.conn.clone()
    }

}