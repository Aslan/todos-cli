#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::Error;
use clap::{App, SubCommand, Arg};

#[derive(Serialize, Deserialize)]
struct TodoItem {
    name: String,
    completed: bool
}

impl TodoItem {
     fn new(name: String) -> TodoItem {
         TodoItem { name: name, completed: false }
     }
}

struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
     fn new(vec: Vec<TodoItem>) -> TodoList {
         TodoList { list: vec }
     }

    fn add(&mut self, name: String) {
        self.list.push(TodoItem::new(name));
    }

    fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn toggle(&mut self, index: usize) {
        self.list[index].completed = !self.list[index].completed;
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, if item.completed { 'x' } else { ' ' }, item.name);
        }
    }

    fn load(path: &str) -> Result<TodoList, Error> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(TodoList::new(serde_json::from_str(&data).unwrap()))
    }

    fn save(&self, path: String) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(&self.list).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes())
    }
}

enum Command {
    Help,
    Get,
    Add(String),
    Remove(usize),
    Toggle(usize)
}

fn print_todos_if_ok(res: Result<(), Error>, todos: TodoList) {
    if res.is_ok() {
        todos.print();
    } else {
        println!("Error: {}", res.err().unwrap());
    }
}

fn main() {
    let mut app = App::new("Todos CLI")
        .version("1.0")
        .author("Pepe Becker <mail@pepebecker.com>")
        .about("Create and manage todos using the command line")
        .subcommand(SubCommand::with_name("add")
            .alias("new").alias("create").about("creates a new todo")
            .arg(Arg::with_name("todo").required(true)))
        .subcommand(SubCommand::with_name("toggle")
            .about("toggles a todo between completed and not completed")
            .arg(Arg::with_name("todo_num").required(true)))
        .subcommand(SubCommand::with_name("remove")
            .alias("delete").alias("rm").about("removes a todo")
            .arg(Arg::with_name("todo_num").required(true)))
        .subcommand(SubCommand::with_name("get")
            .alias("list").alias("ls").about("lists all todos"));

    let mut command = Command::Help;

    match app.clone().get_matches().subcommand() {
        ("add", Some(create)) => {
            if let Some(todo) = create.value_of("todo") {
                command = Command::Add(todo.to_string());
            }
        },
        ("toggle", Some(toggle)) => {
            if let Some(Ok(n)) = toggle.value_of("todo_num").map(|s| s.parse::<usize>()) {
                command = Command::Toggle(n);
            }
        },
        ("remove", Some(remove)) => {
            if let Some(Ok(n)) = remove.value_of("todo_num").map(|s| s.parse::<usize>()) {
                command = Command::Remove(n);
            }
        },
        ("get", _) => command = Command::Get,
        _ => command = Command::Help
    }

    let path = "todos.json".to_string();
    let mut todo_list = TodoList::load(path.as_str()).unwrap();

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add(task);
            print_todos_if_ok(todo_list.save(path), todo_list);
        },
        Command::Remove(index) => {
            todo_list.remove(index);
            print_todos_if_ok(todo_list.save(path), todo_list);
        },
        Command::Toggle(index) => {
            todo_list.toggle(index);
            print_todos_if_ok(todo_list.save(path), todo_list);
        },
        Command::Help => {
            let _ = app.print_help();
        }
    }
}
