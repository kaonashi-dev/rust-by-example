# Todo CLI (JSON-backed)

A minimal todo CLI written in Rust. Tasks are stored in a JSON file.

## Build and Run

- From this `03` directory, run:
  - `cargo run -- <command> [args]`
- By default, the database file is `./todos.json` in the current working directory. Override with env var `TODO_DB`.

## Usage

- `add <title> [description]` : Add a new todo.
- `list [--all|--pending|--done]` : List todos (default: all).
- `done <index>` : Mark a todo as completed.
- `undone <index>` : Mark a todo as not completed.
- `remove <index>` : Remove a todo.
- `edit <index> <title> [description]` : Edit a todo.

Indices are 1-based. Use `list` to see the current indices.

## Environment

- `TODO_DB=path/to/file.json` : Override the path to the JSON database.

## Notes

- This project uses `serde` and `serde_json` for JSON serialization/deserialization.
- The JSON schema is simply an array of objects like:
  ```json
  [
    { "title": "Task A", "description": "Details", "completed": false }
  ]
  ```
