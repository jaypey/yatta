use chrono::{DateTime, Local};
use rusqlite::{params, Connection};

pub struct Task {
    pub id: i64,
    pub description: String,
    pub date_created: DateTime<Local>,
    pub date_ended: DateTime<Local>,
    pub expected_time: i32,
    pub elapsed_time: i32,
    pub project_id: i64,
}

impl Task {
    pub fn new(
        description: str,
        date_created: DateTime<Local>,
        expected_time: i32,
        project_id: i32,
    ) -> Self {
        Task {
            id: 0,
            description: description,
            date_created: Local::now(),
            date_ended: Local::now(),
            expected_time: expected_time,
            elapsed_time: 0,
            project_id: project_id,
        }
    }

    pub fn create(&self, &conn: &Connection) -> Result<String, String> {
        self.id = conn.last_insert_rowid() + 1; // Add after Ok execute
        return conn.execute(format!(
            "
                        INSERT INTO tasks (description, dateCreated, dateEnded, expectedTime, elapsedTime, project_id)
                        VALUES {}, {}, NULL, {}, {}, {}
                        ",
            self.description,
            self.date_created.to_string(),
            self.expected_time,
            self.elapsed_time,
            self.project_id
        ));
    }

    pub fn update(&self, &conn: &Connection) {
        self.id = conn.last_insert_rowid();
        match conn.execute(
            "
                        UPDATE Tasks
                        SET description = ?1, dateEnded = ?2, elapsedTime = ?3
                        WHERE id = ?4
                        ",
            params![
                self.description,
                self.date_ended,
                self.elapsed_time,
                self.id
            ],
        ) {
            Ok(updated) => println!("Updated"),
            Err(err) => println!("Error"),
        };
    }
}
