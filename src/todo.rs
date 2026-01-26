use chrono::NaiveDate;
use std::io;
use std::io::*;
#[derive(Debug)]
struct Task {
    id: u16,
    task: String,
    exp_date: NaiveDate,
}

impl Task {
    fn new(id: u16, task: String, year: u32, date: u32, month: u32) -> Self {
        Task {
            id,
            task,
            exp_date: NaiveDate::from_ymd_opt(year as i32, date, month).unwrap(),
        }
    }
}

fn greet() {
    println!("<------ This is a to-do list ------>");
}

fn print_options() {
    println!("Choices:");
    println!("1. Add to list");
    println!("2. Remove from list");
    println!("3. View all");
    println!("4. Quit");
    println!(">>>");
    io::stdout().flush().unwrap();
}

// <------ FUNCTIONS MODIFYING THE LIST ------>
