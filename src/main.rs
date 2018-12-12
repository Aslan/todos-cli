#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use clap::{App, SubCommand, Arg};
use serde_json::Value;

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
     fn new() -> TodoList {
         TodoList { list: Vec::new() }
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

    fn load(path: String) -> TodoList {
        let mut file = File::open(path).expect("Unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Unable to read content from file");

        let mut list = TodoList::new();

        let value: Value = serde_json::from_str(&data).expect("Unable to parse file content to json");
        for item in value.as_array().unwrap() {
            let name = item.get("name").expect("Unable to get name from json")
                           .as_str().expect("Unable to cast name to str");
            let completed = item.get("completed").expect("Unable to get completed from json")
                            .as_bool().expect("Unable to cast completed to bool");
            list.add(name.to_string());
            if completed {
                list.toggle(list.list.len() - 1);
            }
        }

        list
    }

    fn save(&self, path: String) {
        let json = serde_json::to_string_pretty(&self.list).expect("Unable to parse todos to json");
        let mut file = File::create(path).expect("Unable to create json file");
        file.write_all(json.as_bytes()).expect("Unable to write json to file");
    }
}

enum Command {
    Help,
    Get,
    Add(String),
    Remove(usize),
    Toggle(usize)
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

    let mut todo_list = TodoList::load("todos.json".to_string());

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add(task);
            todo_list.save("todos.json".to_string());
            todo_list.print();
        },
        Command::Remove(index) => {
            todo_list.remove(index);
            todo_list.save("todos.json".to_string());
            todo_list.print();
        },
        Command::Toggle(index) => {
            todo_list.toggle(index);
            todo_list.save("todos.json".to_string());
            todo_list.print();
        },
        Command::Help => {
            let _ = app.print_help();
        }
    }
}
