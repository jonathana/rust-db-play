extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde_json::{Map, Value};
use serde_rusqlite::*;

fn main() -> Result<()> {
    let manager = SqliteConnectionManager::memory(); //file(":memory:");
    let pool = r2d2::Pool::new(manager).unwrap();
    let conn = pool.get().unwrap();

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
        params![],
    )?;

    conn.execute(
        "INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
        params!["Steven".to_string(), time::get_time(), None as Option<Vec<u8>>],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person")?;
    let perjson_iter = stmt.query_map(params![], |row| {
        Ok(from_row::<Map<String,Value>>(row).unwrap())
    })?;

    for perjson in perjson_iter {
        println!("Found person {:?}", serde_json::to_string(&perjson.unwrap()).unwrap());
    }
    Ok(())
}
