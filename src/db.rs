use crate::todo::*;
use chrono::NaiveDate;
use sqlx::PgPool;
use sqlx::Row;

pub struct DataBase {
    pool: PgPool,
}

impl DataBase {
    pub async fn create_connection(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    pub async fn insert_new_task(self, task: &Task) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO tasks (id,task,exp_date) VALUES ($1,$2,$3)")
            .bind(&task.id)
            .bind(&task.task)
            .bind(&task.exp_date)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_task(&self, id: &u32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM rust_todo WHERE id=$1")
            .bind(*id as i64)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn get_all_tasks(&self) -> Result<Vec<Vec<String>>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM rust_todo")
            .fetch_all(&self.pool)
            .await?;

        let all_tasks = rows
            .iter()
            .map(|row| {
                vec![
                    row.try_get::<i32, _>(0).to_string(),
                    row.get::<String, _>(1),
                    row.get::<NaiveDate, _>(2).to_string(),
                ]
            })
            .collect();

        Ok(all_tasks)
    }
}
