use std::io::{self, Write};

pub fn run() {
    let mut todo_list = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        match command {
            "quit" => break,
            "list" => list_tasks(&todo_list),
            "add" => add_task(&mut todo_list),
            "remove" => remove_task(&mut todo_list),
            _ => println!("Unknown command: {}", command),
        }
    }
}

fn list_tasks(todo_list: &Vec<String>) {
    println!("Tasks :");

    for task in todo_list {
        println!("- {}", task);
    }
}

fn add_task(todo_list: &mut Vec<String>) {
    let mut input = String::new();

    print!("Enter task: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let task = input.trim().to_string();

    todo_list.push(task);

    println!("Task added!");
}

fn remove_task(todo_list: &mut Vec<String>) {
    let mut input = String::new();

    print!("Enter task index to remove: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap();

    if index >= todo_list.len() {
        println!("Invalid index!");
    } else {
        todo_list.remove(index);
        println!("Task removed!");
    }
}
