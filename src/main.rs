use serde_json::{Map, Value, json};
// use std::collections::HashMap;
use std::fs;
// use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::*;
use std::process::Command;
// use std::thread;
// use std::time::Duration;

#[derive(Debug)]
struct Task {
    id: String,
    task: String,
    exp_date: String,
}

fn print_tasks() -> Result<()> {
    let file_path = "data/main.json";

    // Safe file read: missing file treated as empty
    let data = fs::read_to_string(file_path).unwrap_or_else(|_| String::new());

    // Exit early if file is empty or whitespace
    if data.trim().is_empty() {
        println!("No tasks found");
        return Ok(());
    }

    // Parse JSON safely
    let json: Value = match serde_json::from_str(&data) {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid JSON, starting empty");
            Value::Object(Map::new())
        }
    };

    // Only print if it’s an object
    if let Value::Object(map) = json {
        if map.is_empty() {
            println!("No tasks found");
        } else {
            for (key, val) in map {
                println!("ID: {key}");
                println!("Contents: {}", val);
            }
        }
    } else {
        println!("JSON is not an object");
    }

    Ok(())
}

fn clear_terminal() {
    Command::new("clear").status();
    io::stdout().flush().unwrap();
}

fn add_to_file(obj_task: &Task) -> std::io::Result<()> {
    let mut root: Value = match fs::read_to_string("data/main.json") {
        Ok(data) => serde_json::from_str(&data)?,
        Err(_) => json!({}),
    };

    if let Value::Object(map) = &mut root {
        map.insert(
            obj_task.id.to_string(),
            json!({"task":obj_task.task,"exp_date":obj_task.exp_date}),
        );
    }
    let output = serde_json::to_string_pretty(&root).unwrap();
    fs::write("data/main.json", output)?;

    Ok(())
}

fn print_options() {
    println!("Choices:");
    println!("        1. Print all tasks");
    println!("        2. Add to list");
    println!("        3. Remove from list");
    println!("        4. Quit");
    print!("Choice: ");
    io::stdout().flush().unwrap();
}

fn delete_from_list() -> std::io::Result<()> {
    //load file data
    let file = String::from("data/main.json");
    let data = fs::read_to_string(&file)?;

    let mut json_data: Value = serde_json::from_str(&data).expect("");
    let obj = json_data.as_object_mut().unwrap();

    if data.trim().is_empty() {
        println!("No tasks found");
        return Ok(());
    }

    let mut id_to_delete = String::new();
    // let mut found:bool = false;

    println!("Enter the task id\n>>>");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut id_to_delete).expect("");
    let id_to_delete = id_to_delete.trim();

    if obj.remove(id_to_delete).is_some() {
        let new_json = serde_json::to_string_pretty(&obj).expect("");
        fs::write(file, new_json)?;
        println!("Task deleted successfully");
    } else {
        println!("No task with id: {}", id_to_delete);
    }

    Ok(())
}

fn add_to_list() {
    print!("Create an id for the task\n>>>");
    io::stdout().flush().unwrap();

    let mut task_id: String = String::new();
    io::stdin().read_line(&mut task_id).unwrap();

    print!("\nEnter the task\n>>> ");
    io::stdout().flush().unwrap();

    let mut taskChoice = String::new();
    io::stdin().read_line(&mut taskChoice).expect("");

    println!();

    print!("Enter the expiration date\n>>> ");
    io::stdout().flush().unwrap();

    let mut exp_date_input = String::new();
    io::stdin().read_line(&mut exp_date_input).expect("");

    let current_task: Task = Task {
        id: task_id.trim().to_string(),
        task: taskChoice,
        exp_date: exp_date_input,
    };
    add_to_file(&current_task);

    clear_terminal();
}

fn main() {
    clear_terminal();
    println!("<--------This is a to-do list-------->\n");
    print_options();

    // let mut tasks: HashMap<String, Task> = HashMap::new();

    // let mut tasks_counter: i32 = 1;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        let choice = input.chars().next().expect("");

        match choice {
            '1' => {
                clear_terminal();
                io::stdout().flush().unwrap();
                let _ = print_tasks();
            }
            '2' => {
                clear_terminal();
                io::stdout().flush().unwrap();
                add_to_list()
            }
            '3' => {
                clear_terminal();
                let _ = print_tasks();
                let _ = delete_from_list();
            }
            '4' => return,
            _ => println!("No option for{choice}"),
        }
        print_options();
    }
}
