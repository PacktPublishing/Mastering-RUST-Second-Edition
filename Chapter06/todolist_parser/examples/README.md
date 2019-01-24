
To see the custom errors, try running the `basics.rs` example by initializing `TodoList`
in one of the following ways:

* `let todos = TodoList::get_todos("examples/malformed_todo");` - The case where we have a malformed todo.
* `let todos = TodoList::get_todos("examples/todos");` - The case where our `todos` file doesn't exist.
