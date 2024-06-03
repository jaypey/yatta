use chrono::{DateTime, Local};
use rusqlite::Connection;

pub struct Project {
    pub id: i64,
    pub name: str,
    pub date_created: DateTime<Local>,
    pub completed: bool,
}

impl Project {
    pub fn new(name: str) -> Self {
        Project {
            id: 0,
            name: name,
            date_created: Local::now(),
            completed: false,
        }
    }

    pub fn create(&self, &conn: Connection) -> Result<String, String> {
        return conn.execute(format!(
            "
                        INSERT INTO projects (name, dateCreated, completed)
                        VALUES {}, {}, 0
                        ",
            self.name,
            self.date_created.to_string()
        ));
    }
}
