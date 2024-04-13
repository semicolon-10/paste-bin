use std::sync::{Arc, Mutex};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rusqlite::{Connection, Error, params};

#[derive(Clone)]
pub struct DBService {
    db: Arc<Mutex<Connection>>,
}

impl DBService {
    pub async fn new() -> Result<Self,Error> {
        let db = Connection::open("content.db")
            .expect("Failed to open database");

        db.execute(
            "CREATE TABLE IF NOT EXISTS pastebin (token TEXT PRIMARY KEY, content TEXT)",
            params![]
        )?;

        Ok(DBService { db: Arc::new(Mutex::new(db)) })
    }

    pub async fn get_content(&self, token: String) -> Result<String, Error> {
        let connection = self.db.lock()
            .unwrap();

        connection
            .query_row(
                "SELECT content FROM pastebin WHERE token = ?",
                params![token],
                |row| row.get::<_, String>(0),
            )
    }

    pub async fn store_content(&self, content: String) -> Result<String, Error> {
        let token : String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();

        let connection = self.db.lock().unwrap();

        match connection.execute(
            "INSERT INTO pastebin (token,content) VALUES (?,?)",
            params![&token, &content],
        ) {
            Ok(_) => Ok(token),
            Err(ex) => Err(ex.into())
        }
    }
}
