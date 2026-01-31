use chrono::NaiveDate;
use std::io;
use std::io::*;
use std::process;
use std::{collections::HashSet, task};

use crate::db::{self, DataBase};
#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub task: String,
    pub exp_date: NaiveDate,
}

impl Task {
    pub fn new(id: i32, task: String, year: u32, date: u32, month: u32) -> Self {
        Task {
            id,
            task,
            exp_date: NaiveDate::from_ymd_opt(year as i32, month, date).unwrap(),
        }
    }
}

pub fn greet() {
    println!("<------ This is a to-do list ------>");
}
fn user_input() -> String {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("");
    choice.trim().to_string()
}

pub async fn impl_choice(db: &db::DataBase) {
    loop {
        let choice: String = user_input();

        match choice.as_str() {
            "1" => {
                let task: Task = add_to_list();
                match db.insert_new_task(&task).await {
                    Ok(_) => println!("Task inserted successfully\n>>> "),
                    Err(e) => println!("Failed to insert new task: {e}\n>>> "),
                }
            }
            "2" => {
                let id = id_to_delete();
                match db.delete_task(&id).await {
                    Ok(_) => println!("Task deleted successfully\n>>> "),
                    Err(e) => println!("Failed to delete task: {e}\n>>> "),
                }
            }
            "3" => match db.get_all_tasks().await {
                Ok(tasks) => {
                    print_tasks(tasks);
                }
                Err(e) => {
                    println!("Failed to fetch tasks: {}\n>>> ", e)
                }
            },
            "4" => process::exit(0),
            _ => {
                println!("No match found:{}", &choice)
            }
        }
    }
}

pub fn print_options() {
    println!("Choices:");
    println!("1. Add task to list");
    println!("2. Remove from list");
    println!("3. View all");
    println!("4. Quit");
    print!(">>> ");
    io::stdout().flush().unwrap();
}

pub fn flush_terminal() {
    io::stdout().flush().unwrap();
}
// <------ FUNCTIONS MODIFYING THE LIST ------>

pub fn print_tasks(tasks: Vec<Task>) {
    for t in &tasks {
        println!("{},{},{}", t.id, t.task, t.exp_date);
    }
}

pub fn add_to_list() -> Task {
    //<------ ID PART ----->
    print!("Create id for the task\n>>> ");
    flush_terminal();
    let mut id = String::new();
    let mut id_int: i32 = 0;
    loop {
        io::stdin().read_line(&mut id).expect("");
        id = id.trim().to_string();

        match id.parse::<i32>() {
            Ok(num) => {
                if num < 0 {
                    println!("The id must be more than 0");
                    continue;
                }
                id_int = num;
                break;
            }
            Err(_) => {
                println!("Failed to parse string into u16");
                id.clear();
                continue;
            }
        }
    }

    //<----- TASK PART ----->

    println!("Create a task\n>>> ");
    flush_terminal();

    let mut task: String = String::new();
    io::stdin().read_line(&mut task).expect("");

    //<---------- EXP-DATE PART ---------->

    println!("Enter the exp-date\n>>> ");
    flush_terminal();

    let mut year_int: u32 = 0;
    let mut date_int: u32 = 0;
    let mut month_string = String::new();
    let mut month_int: u32 = 0;

    print!("Enter the full year\n>>> ");
    flush_terminal();

    // <----- year_string ----->
    loop {
        let mut year_string = String::new();
        io::stdin().read_line(&mut year_string).expect("");
        let _ = year_string.trim();

        let year_string = year_string.trim();

        match year_string.parse::<u32>() {
            Ok(num) => {
                if year_string.len() != 4 {
                    println!("The year must be exactly 4 digits");
                    continue;
                }

                year_int = num;
                break;
            }
            Err(_) => {
                println!("The year must contain only numeric values\n>>>");
                continue;
            }
        }
    }
    // <----- DATE ----->
    print!("Enter a valid date\n>>> ");
    loop {
        let mut date = String::new();
        io::stdin().read_line(&mut date).expect("");

        let date = date.trim();

        match date.parse::<u32>() {
            Ok(date_match_case) => {
                if date.len() < 1 || date.len() > 2 {
                    println!("Date must be varying from 1 to 31\n>>> ");
                    continue;
                }
                date_int = date_match_case;
                break;
            }
            Err(_) => {
                println!("Date must be a valid number\n>>> ");
                continue;
            }
        }
    }

    // <----- MONTH ----->
    print!("Enter a month (using month's name)\n>>> ");

    loop {
        month_string.clear();
        io::stdin().read_line(&mut month_string).expect("");

        match month_string.trim().to_uppercase().as_str() {
            "JANUARY" => {
                month_int = 1;
                break;
            }
            "FEBRUARY" => {
                month_int = 2;
                break;
            }
            "MARCH" => {
                month_int = 3;
                break;
            }
            "APRIL" => {
                month_int = 4;
                break;
            }
            "MAY" => {
                month_int = 5;
                break;
            }
            "JUNE" => {
                month_int = 6;
                break;
            }
            "JULY" => {
                month_int = 7;
                break;
            }
            "AUGUST" => {
                month_int = 8;
                break;
            }
            "SEPTEMBER" => {
                month_int = 9;
                break;
            }
            "OCTOBER" => {
                month_int = 10;
                break;
            }
            "NOVEMBER" => {
                month_int = 11;
                break;
            }
            "DECEMBER" => {
                month_int = 12;
                break;
            }
            _ => {
                println!("Enter a valid month");
                continue;
            }
        }
    }
    Task::new(id_int, task, year_int, date_int, month_int)
}

fn id_to_delete() -> i32 {
    let mut id_int: i32 = 0;
    loop {
        let mut id_string = String::new();
        io::stdin().read_line(&mut id_string).expect("");

        match id_string.trim().parse::<i32>() {
            Ok(num) => {
                if num < 0 {
                    println!("The id must be more than 0\n>>> ");
                    continue;
                }
                id_int = num;
                break;
            }
            Err(_) => {
                println!("The id must be a valid numeric value\n>>> ");
                continue;
            }
        }
    }
    id_int
}
