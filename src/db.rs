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

    pub async fn insert_new_task(&self, task: &Task) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO tasks (id,task,exp_date) VALUES ($1,$2,$3)")
            .bind(&task.id)
            .bind(&task.task)
            .bind(&task.exp_date)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_task(&self, id: &u32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tasks WHERE id=$1")
            .bind(*id as i64)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM tasks")
            .fetch_all(&self.pool)
            .await?;

        let all_tasks: Vec<Task> = rows
            .iter()
            .map(|row| Task {
                id: row.get::<i32, _>(0),
                task: row.get::<String, _>(1),
                exp_date: row.get::<NaiveDate, _>(2),
            })
            .collect();

        Ok(all_tasks)
    }
}
