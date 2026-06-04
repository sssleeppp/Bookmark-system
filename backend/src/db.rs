use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub type DbPool = Arc<Mutex<Connection>>;

pub fn init_db() -> DbPool {
    let db_dir = Path::new(".local");
    std::fs::create_dir_all(db_dir).expect("Failed to create .local directory");

    let db_path = db_dir.join("bookmark.db");
    let conn = Connection::open(&db_path).expect("Failed to open SQLite database");

    conn.execute_batch(
        "
        DROP TABLE IF EXISTS bookmark;
        DROP TABLE IF EXISTS category;
        DROP TABLE IF EXISTS user;

        CREATE TABLE user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        );

        CREATE TABLE category (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            parent_id INTEGER,
            sort_order INTEGER DEFAULT 0
        );

        CREATE TABLE bookmark (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            category_id INTEGER,
            user_id INTEGER NOT NULL
        );

        INSERT INTO user (username, password) VALUES ('admin', '123456');
        ",
    )
    .expect("Failed to initialize database schema");

    Arc::new(Mutex::new(conn))
}
