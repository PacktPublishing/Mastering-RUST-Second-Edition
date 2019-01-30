// postgres_demo/src/main.rs

use postgres::{Connection, TlsMode};

const DROP_TABLE: &str = "DROP TABLE IF EXISTS books";
const CONNECTION: &str = "postgres://postgres:postgres@localhost:5432";
const CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS books 
                            (id SERIAL PRIMARY KEY,
                            title VARCHAR NOT NULL,
                            author VARCHAR NOT NULL,
                            year SERIAL)";

fn reset_db(conn: &Connection) {
    let _ = conn.execute(DROP_TABLE, &[]).unwrap();
    let _ = conn.execute(CREATE_TABLE, &[]).unwrap();
}

#[derive(Debug)]
struct Book {
    id: i32,
    title: String,
    author: String,
    year: i32
}

fn main() {
    let conn = Connection::connect(CONNECTION, TlsMode::None).unwrap();
    reset_db(&conn);

    let book = Book {
        id: 3,
        title: "A programmers introduction to mathematics".to_string(),
        author: "Dr. Jeremy Kun".to_string(),
        year: 2018
    };

    conn.execute("INSERT INTO books (id, title, author, year) VALUES ($1, $2, $3, $4)",
                 &[&book.id, &book.title, &book.author, &book.year]).unwrap();

    for row in &conn.query("SELECT id, title, author, year FROM books", &[]).unwrap() {
        let book = Book {
            id: row.get(0),
            title: row.get(1),
            author: row.get(2),
            year: row.get(3)
        };
        println!("{:?}", book);
    }
}
