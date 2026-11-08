use std::fs;

use pass_man_v2::database::DatabasePool;
use pass_man_v2::models::PasswordEntry;


fn main() {

    let mut data_dir = dirs::data_dir().ok_or("could not find local data directory");
    data_dir.push("pass_man");
    fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    data_dir.push("pass_man.db3");

    let db_path = data_dir.to_string().ok_or("Invalid database path string")?;

    println!("Initializing database at: {}", db_path);

    let pool = DatabasePool::new(db_path)?;

    println!("Connected to database!");
}


