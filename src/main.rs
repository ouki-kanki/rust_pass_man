use std::fs;

use pass_man_v2::database::DatabasePool;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut data_dir = dirs::data_dir().ok_or("could not find local data directory")?;
    data_dir.push("pass_man");
    fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    data_dir.push("pass_man.db3");

    let db_path = data_dir.to_str().ok_or("Invalid database path string")?;

    println!("Initializing database at: {}", db_path);

    let pool = DatabasePool::new(db_path)?;

    println!("Connected to database!");


    Ok(())
}


