// r2d2_demo/src/main.rs

use std::thread;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use std::time::Duration;

const DROP_TABLE: &str = "DROP TABLE IF EXISTS books";

const CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS books 
                            (id SERIAL PRIMARY KEY,
                            title VARCHAR NOT NULL,
                            author VARCHAR NOT NULL,
                            year SERIAL)";

#[derive(Debug)]
struct Book {
    id: i32,
    title: String,
    author: String,
    year: i32
}

fn main() {
    let manager = PostgresConnectionManager::new("postgres://postgres:postgres@localhost:5432",
                                                 TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();
    let conn = pool.get().unwrap();

    let _ = conn.execute(DROP_TABLE, &[]).unwrap();
    let _ = conn.execute(CREATE_TABLE, &[]).unwrap();
    
    thread::spawn(move || {
        let book = Book {
            id: 3,
            title: "A programmers introduction to mathematics".to_string(),
            author: "Dr. Jeremy Kun".to_string(),
            year: 2018
        };
        conn.execute("INSERT INTO books (id, title, author, year) VALUES ($1, $2, $3, $4)",
                    &[&book.id, &book.title, &book.author, &book.year]).unwrap();                                         
    });

    thread::sleep(Duration::from_millis(100));
    for _ in 0..8 {
        let conn = pool.get().unwrap();
        thread::spawn(move || {
            for row in &conn.query("SELECT id, title, author, year FROM books", &[]).unwrap() {
                let book = Book {
                    id: row.get(0),
                    title: row.get(1),
                    author: row.get(2),
                    year: row.get(3)
                };
                println!("{:?}", book);
            }
        });
    }
}
