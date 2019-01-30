// rusqlite_demo/src/main.rs

use std::io;
use std::io::BufRead;

use rusqlite::Error;
use rusqlite::{Connection, NO_PARAMS};

const CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS books 
                            (id INTEGER PRIMARY KEY,
                            title TEXT NOT NULL,
                            author TEXT NOT NULL,
                            year INTEGER NOT NULL)";

#[derive(Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
    year: u16,
}

fn extract_ok(p: Result<Book, Error>) -> Option<Book> {
    if p.is_ok() {
        Some(p.unwrap())
    } else {
        None
    }
}

fn insert(conn: &Connection) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let elems = line.unwrap();
        let elems: Vec<&str> = elems.split(",").collect();
        if elems.len() > 2 {
            let _ = conn.execute(
                "INSERT INTO books (id, title, author, year) VALUES (?1, ?2, ?3, ?4)",
                &[&elems[0], &elems[1], &elems[2], &elems[3]],
            );
        }
    }
}

fn init_database(conn: &Connection) {
    conn.execute(CREATE_TABLE, NO_PARAMS).unwrap();
}

fn query(conn: &Connection) {
    let mut stmt = conn
        .prepare("SELECT id, title, author, year FROM books WHERE year >= ?1")
        .unwrap();
    let movie_iter = stmt
        .query_map(&[&2013], |row| Book {
            id: row.get(0),
            title: row.get(1),
            author: row.get(2),
            year: row.get(3),
        })
        .unwrap();

    for movie in movie_iter.filter_map(extract_ok) {
        println!("Found book {:?}", movie);
    }
}

fn main() {
    let conn = Connection::open("./books").unwrap();
    init_database(&conn);
    insert(&conn);
    query(&conn);
}
