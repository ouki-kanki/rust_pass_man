use crate::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

impl DatabasePool {
    pub fn new(database_url: &str) -> Result<Self> {

        let manager = SqliteConnectionManager::file(database_url);
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)?;

        let conn = pool.get()?;
        create_tables(&conn)?;

        Ok(self.pool.get()?)
    }
}


fn create_tables(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS password_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            app_name TEXT NOT NULL,
            username TEXT,
            email TEXT,
            password TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}
