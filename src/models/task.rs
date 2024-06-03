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
        description: String,
        date_created: DateTime<Local>,
        expected_time: i32,
        project_id: i64,
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

    pub fn create(&mut self, conn: Connection){
        match conn.execute(
            "
                        INSERT INTO tasks (description, dateCreated, dateEnded, expectedTime, elapsedTime, project_id)
                        VALUES ?1, ?2, NULL, ?3, {?4, ?5
                        ",
            params![self.description,
            self.date_created.to_string(),
            self.expected_time,
            self.elapsed_time,
            self.project_id]
        ){
            Ok(created) => {
                self.id = conn.last_insert_rowid();
                println!("Created");
            },
            Err(err) => println!("Error")
        };
    }

    pub fn update(&self, conn: Connection) {
        match conn.execute(
            "
                        UPDATE Tasks
                        SET description = ?1, dateEnded = ?2, elapsedTime = ?3
                        WHERE id = ?4
                        ",
            params![
                self.description,
                self.date_ended.to_string(),
                self.elapsed_time,
                self.id
            ],
        ) {
            Ok(updated) => println!("Updated"),
            Err(err) => println!("Error"),
        };
    }
}
