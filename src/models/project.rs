use chrono::{DateTime, Local};
use rusqlite::{params, Connection};

pub struct Project {
    pub id: i64,
    pub name: String,
    pub date_created: DateTime<Local>,
    pub completed: bool,
}

impl Project {
    pub fn new(name: String) -> Self {
        Project {
            id: 0,
            name: name,
            date_created: Local::now(),
            completed: false,
        }
    }

    pub fn create(&mut self, conn: Connection){
        match conn.execute(
            "
                        INSERT INTO projects (name, dateCreated, completed)
                        VALUES {}, {}, 0
                        ",
                        params![
            self.name,
            self.date_created.to_string()]
        ){
            Ok(ajouts) => {
                self.id = conn.last_insert_rowid();
                println!("Ajouts: {}", ajouts);
            },
            Err(err) => println!("Erreur: {}", err.to_string())
        }
    }
}
