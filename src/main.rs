use crate::db::DataBase;

mod db;
mod todo;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    todo::greet();
    todo::print_options();
    let db =
        DataBase::create_connection("postgres://vlados3:vlados3@localhost:5432/rust_todo").await?;

    todo::impl_choice(&db).await;
    Ok(())
}
