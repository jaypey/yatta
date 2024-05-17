use clap::Parser;
use directories::ProjectDirs;
use rusqlite::Connection;

const YATTA_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser, Debug)]
#[command(version, about = "Yet Another Time Tracking App", long_about = None)]
struct Cli{
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.count{
        println!("Hello {}!", cli.name)
    }
}

fn init_database() -> Result<String, String>{
    if let Some(proj_dirs) = ProjectDirs::from("org", "JEEP", YATTA_NAME) {
        let path = proj_dirs.data_dir();
        if path.exists(){
            let conn= Connection::open(path.join("yatta.db")).unwrap();
            conn.execute("
            CREATE TABLE projects (
                id	INTEGER NOT NULL UNIQUE,
                name TEXT,
                dateCreated	TEXT,
                completed	INTEGER,
                PRIMARY KEY(id AUTOINCREMENT)
            );", ()).unwrap();
            conn.execute("
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
                );", ()).unwrap();

            return Ok("Success".to_string());
        }else{
            return Ok("Success".to_string());
        }
    }
    return Err("Can't open path".to_string());
}
