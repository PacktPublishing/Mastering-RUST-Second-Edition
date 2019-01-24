// todolist_parser/examples/basics.rs

extern crate todolist_parser;

use todolist_parser::TodoList;

fn main() {
    let todos = TodoList::get_todos("examples/todo");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}", e.description());
            println!("{:?}", e)
        }
    }
}
