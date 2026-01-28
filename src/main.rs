use crate::db::DataBase;

mod db;
mod todo;

fn main() {
    let db = DataBase::create_connection("postgres://vlados3:vlados3@localhost:5432/rust_todo");
    todo::choice();
}
