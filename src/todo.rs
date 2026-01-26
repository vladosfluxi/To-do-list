use chrono::NaiveDate;
use std::collections::HashSet;
use std::io;
use std::io::*;

#[derive(Debug)]
struct Task {
    id: String,
    task: String,
    exp_date: NaiveDate,
}

impl Task {
    fn new(id: String, task: String, year_string: u32, date: u32, month_string: u32) -> Self {
        Task {
            id,
            task,
            exp_date: NaiveDate::from_ymd_opt(year_string as i32, date, month_string).unwrap(),
        }
    }
}

fn greet() {
    println!("<------ This is a to-do list ------>");
}

fn print_options() {
    println!("Choices:");
    println!("1. Add task to list");
    println!("2. Remove from list");
    println!("3. View all");
    println!("4. Quit");
    println!(">>>");
    io::stdout().flush().unwrap();
}

fn flush_terminal() {
    io::stdout().flush().unwrap();
}
// <------ FUNCTIONS MODIFYING THE LIST ------>

fn add_to_list() {
    //<------ ID PART ----->
    println!("Create id for the task\n>>> ");
    flush_terminal();
    let mut id = String::new();
    loop {
        io::stdin().read_line(&mut id).expect("");
        id.trim();

        match id.parse::<u16>() {
            Ok(_) => break,
            Err(_) => {
                println!("Failed to parse string into u16");
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

    let mut year_string = String::new();
    let mut year_int: u32 = 0;
    let mut date = String::new();
    let mut date_int: u32 = 0;
    let mut month_string = String::new();
    let mut month_int: u32 = 0;

    println!("Enter the full year_string\n>>> ");
    flush_terminal();

    // <----- year_string ----->
    loop {
        io::stdin().read_line(&mut year_string).expect("");
        year_string.trim();

        if year_string.parse::<u32>().is_err() {
            println!("Year must contain only numeric values\nEnter the year again\n>>> ");
            continue;
        }
        let year_int_len = year_string.len();
        year_int = year_string.parse().expect("");

        if (year_int_len != 4) {
            println!("The year value must be valid");
            continue;
        }

        break;
    }

    // <----- DATE ----->
    println!("Enter a valid date\n>>> ");
    loop {
        io::stdin().read_line(&mut date).expect("");

        if let Err(_) = date.trim().parse::<u32>() {
            println!("Failed to parse string to u32.\nEnter a valid date\n>>> ");
            continue;
        }

        let date_len = date.to_string().len();

        if date_len < 1 || date_len > 2 {
            println!("Date must be a valid number.\nEnter a valid date\n>>> ");
            continue;
        }

        date_int = date.parse::<u32>().unwrap();

        break;
    }

    // <----- MONTH ----->
    println!("Enter a month (using month's name)");

    loop {
        io::stdin().read_line(&mut month_string).expect("");

        match month_string.trim().to_uppercase().as_str() {
            "JANUARY" => {
                month_int = 1;
            }
            "FERBUARY" => {
                month_int = 2;
            }
            "MARCH" => {
                month_int = 3;
            }
            "APRIL" => {
                month_int = 4;
            }
            "MAY" => {
                month_int = 5;
            }
            "JUNE" => {
                month_int = 6;
            }
            "JULY" => {
                month_int = 7;
            }
            "AUGUST" => {
                month_int = 8;
            }
            "SEPTEMBER" => {
                month_int = 9;
            }
            "OKTOBER" => {
                month_int = 10;
            }
            "NOVEMBER" => {
                month_int = 11;
            }
            "DECEMBER" => {
                month_int = 12;
            }
            _ => {
                println!("Enter a valid month");
                continue;
            }
        }
    }
    let task = Task::new(id, task, year_int, date_int, month_int);
}
