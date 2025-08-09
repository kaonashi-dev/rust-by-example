use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    title: String,
    description: String,
    completed: bool,
}

enum Filter {
    All,
    Pending,
    Done,
}

fn db_path() -> PathBuf {
    if let Ok(path) = env::var("TODO_DB") {
        PathBuf::from(path)
    } else {
        PathBuf::from("todos.json")
    }
}

fn load_db() -> Vec<Todo> {
    let path = db_path();
    if !path.exists() {
        return Vec::new();
    }
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let mut content = String::new();
    if file.read_to_string(&mut content).is_err() {
        return Vec::new();
    }
    if content.trim().is_empty() {
        return Vec::new();
    }
    serde_json::from_str(&content).unwrap_or_else(|_| Vec::<Todo>::new())
}

fn save_db(todos: &Vec<Todo>) -> Result<(), String> {
    let path = db_path();
    let json = serde_json::to_string_pretty(todos).map_err(|e| e.to_string())?;
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(format!("Failed to create directory: {}", e));
            }
        }
    }
    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())
}

fn print_usage() {
    let exe = env::args().next().unwrap_or_else(|| "todo".to_string());
    println!("Todo CLI (JSON-backed)\n");
    println!("Usage:");
    println!("  {} add <title> [description]        Add a new todo", exe);
    println!("  {} list [--all|--pending|--done]    List todos (default: --all)", exe);
    println!("  {} done <index>                     Mark todo as done", exe);
    println!("  {} undone <index>                   Mark todo as not done", exe);
    println!("  {} remove <index>                   Remove a todo", exe);
    println!("  {} edit <index> <title> [desc]      Edit a todo", exe);
    println!("\nEnvironment:");
    println!("  TODO_DB=path/to/file.json           Override DB path (default: ./todos.json)");
}

fn list_todos(todos: &Vec<Todo>, filter: Filter) {
    if todos.is_empty() {
        println!("No todos yet. Add one with: add <title> [description]");
        return;
    }
    for (i, t) in todos.iter().enumerate() {
        let idx = i + 1;
        let status = if t.completed { "✔" } else { " " };
        let show = match filter {
            Filter::All => true,
            Filter::Pending => !t.completed,
            Filter::Done => t.completed,
        };
        if show {
            if t.description.trim().is_empty() {
                println!("[{}] {} - {}", status, idx, t.title);
            } else {
                println!("[{}] {} - {}\n    {}", status, idx, t.title, t.description);
            }
        }
    }
}

fn parse_index(arg: &str) -> Result<usize, String> {
    let idx: usize = arg
        .parse()
        .map_err(|_| format!("Invalid index '{}': must be a positive number", arg))?;
    if idx == 0 {
        return Err("Index must be 1-based (>= 1)".to_string());
    }
    Ok(idx - 1)
}

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        print_usage();
        return;
    }

    let cmd = args.remove(0).to_lowercase();
    match cmd.as_str() {
        "add" => {
            if args.is_empty() {
                eprintln!("Error: 'add' requires at least a <title>.");
                print_usage();
                return;
            }
            let title = args.remove(0);
            let description = if !args.is_empty() {
                args.join(" ")
            } else {
                String::new()
            };
            let mut todos = load_db();
            todos.push(Todo { title, description, completed: false });
            if let Err(e) = save_db(&todos) {
                eprintln!("Failed to save: {}", e);
                return;
            }
            println!("Added todo (#{})", todos.len());
        }

        "list" => {
            let filter = if let Some(flag) = args.get(0) {
                match flag.as_str() {
                    "--pending" => Filter::Pending,
                    "--done" => Filter::Done,
                    _ => Filter::All,
                }
            } else {
                Filter::All
            };
            let todos = load_db();
            list_todos(&todos, filter);
        }

        "done" => {
            if args.is_empty() {
                eprintln!("Error: 'done' requires an <index>.");
                return;
            }
            let idx = match parse_index(&args[0]) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
            let mut todos = load_db();
            if idx >= todos.len() {
                eprintln!("Index out of range. Use 'list' to see items.");
                return;
            }
            todos[idx].completed = true;
            if let Err(e) = save_db(&todos) {
                eprintln!("Failed to save: {}", e);
                return;
            }
            println!("Marked as done (#{}): {}", idx + 1, todos[idx].title);
        }

        "undone" => {
            if args.is_empty() {
                eprintln!("Error: 'undone' requires an <index>.");
                return;
            }
            let idx = match parse_index(&args[0]) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
            let mut todos = load_db();
            if idx >= todos.len() {
                eprintln!("Index out of range. Use 'list' to see items.");
                return;
            }
            todos[idx].completed = false;
            if let Err(e) = save_db(&todos) {
                eprintln!("Failed to save: {}", e);
                return;
            }
            println!("Marked as not done (#{}): {}", idx + 1, todos[idx].title);
        }

        "remove" | "rm" | "del" => {
            if args.is_empty() {
                eprintln!("Error: 'remove' requires an <index>.");
                return;
            }
            let idx = match parse_index(&args[0]) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
            let mut todos = load_db();
            if idx >= todos.len() {
                eprintln!("Index out of range. Use 'list' to see items.");
                return;
            }
            let removed = todos.remove(idx);
            if let Err(e) = save_db(&todos) {
                eprintln!("Failed to save: {}", e);
                return;
            }
            println!("Removed (#{}): {}", idx + 1, removed.title);
        }

        "edit" => {
            if args.len() < 2 {
                eprintln!("Error: 'edit' requires <index> <title> [description].");
                return;
            }
            let idx = match parse_index(&args[0]) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
            let title = args[1].clone();
            let description = if args.len() > 2 {
                args[2..].join(" ")
            } else {
                String::new()
            };
            let mut todos = load_db();
            if idx >= todos.len() {
                eprintln!("Index out of range. Use 'list' to see items.");
                return;
            }
            todos[idx].title = title;
            todos[idx].description = description;
            if let Err(e) = save_db(&todos) {
                eprintln!("Failed to save: {}", e);
                return;
            }
            println!("Updated (#{}).", idx + 1);
        }

        _ => {
            eprintln!("Unknown command: {}", cmd);
            print_usage();
        }
    }
}

