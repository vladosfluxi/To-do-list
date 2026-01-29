use chrono::NaiveDate;
use std::collections::HashSet;
use std::io;
use std::io::*;

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
            exp_date: NaiveDate::from_ymd_opt(year as i32, date, month).unwrap(),
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
    let choice: String = user_input();

    match choice.as_str() {
        "1" => {
            let task: Task = add_to_list();
            db.insert_new_task(&task).await;
        }
        "2" => {
            let id = id_to_delete();
            db.delete_task(&id).await;
        }
        "3" => {}
        "4" => {}
        _ => {}
    }
}

pub fn print_options() {
    println!("Choices:");
    println!("1. Add task to list");
    println!("2. Remove from list");
    println!("3. View all");
    println!("4. Quit");
    println!(">>>");
    io::stdout().flush().unwrap();
}

pub fn flush_terminal() {
    io::stdout().flush().unwrap();
}
// <------ FUNCTIONS MODIFYING THE LIST ------>

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
    println!("Enter a valid date\n>>> ");
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
    println!("Enter a month (using month's name)");

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
