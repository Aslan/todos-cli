#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
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
    Get,
    Add(String),
    Remove(usize),
    Toggle(usize)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "toggle" => Command::Toggle(args[2].parse().expect("Error converting to usize")),
        "remove" => Command::Remove(args[2].parse().expect("Error converting to usize")),
        _ => panic!("You must provide an accepted command")
    };

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
        }
    }
}
