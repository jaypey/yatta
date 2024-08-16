mod models;

use models::project;
use models::project::Project;
use models::task;

use std::{
    error, fs,
    io::{Error, ErrorKind}, ops::Deref,
};

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use rusqlite::Connection;

const YATTA_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(version, about = "Yet Another Time Tracking App", long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Create {
        name: Option<String>
    },
    Add {
        name: Option<String>
    },
    Start{
        name: Option<String>
    },
    Stop{
        name: Option<String>
    },
}


fn init_database() -> Result<&Connection, Error> {
    if let Some(proj_dirs) = ProjectDirs::from("org", "JEEP", YATTA_NAME) {
        let path = proj_dirs.data_dir();
        let path_exists = path.try_exists();
        if path_exists.is_ok_and(|x| x == false) {
            fs::create_dir_all(path)?;
            let conn = Connection::open(path.join("yatta.db")).unwrap();
            conn.execute(
                "
            CREATE TABLE projects (
                id	INTEGER NOT NULL UNIQUE,
                name TEXT,
                dateCreated	TEXT,
                completed	INTEGER,
                PRIMARY KEY(id AUTOINCREMENT)
            );",
                (),
            )
            .unwrap();
            conn.execute(
                "
                CREATE TABLE tasks (
                    id	INTEGER NOT NULL UNIQUE,
                    description	TEXT,
                    dateCreated	TEXT,
                    dateEnded	TEXT,
                    expectedTime	INTEGER,
                    elapsedTime	INTEGER,
                    project_id	INTEGER,
                    FOREIGN KEY(project_id) REFERENCES projects(id),
                    PRIMARY KEY(id AUTOINCREMENT)
                );",
                (),
            )
            .unwrap();

            return Ok(&conn);
        } else {
            return Ok(&conn);
        }
    }
    return Err(Error::new(ErrorKind::Other, "Can't create files"));
}

fn main() {
    let cli = Cli::parse();

    let init_db_result = init_database();
    match init_db_result {
        Ok(conn) => parse_command(&cli.command, conn),
        Err(error_msg) => panic!("{}", error_msg),
    };


}

fn parse_command(command: &Option<Commands>,conn: &Connection) {
    match command {
        Some(Commands::Add { name }) => ,
        Some(Commands::Create { name }) => Project::new(name.as_deref().unwrap().to_string()).create(conn),
        Some(Commands::Start { name }) => todo!(),
        Some(Commands::Stop { name }) => todo!(),
        None => todo!(),
    }
}
