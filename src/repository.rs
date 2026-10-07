use crate::model::TodoEntity;
use chrono::Utc;
use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct TodoRepository {
    pool: MySqlPool,
}

impl TodoRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_all_todos(&self) -> Result<Vec<TodoEntity>, sqlx::Error> {
        sqlx::query_as::<_, TodoEntity>("SELECT * FROM todos")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn find_one_todo_by_id(&self, id: i64) -> Result<Option<TodoEntity>, sqlx::Error> {
        sqlx::query_as::<_, TodoEntity>("SELECT * FROM todos WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_todo(
        &self,
        title: String,
        content: String,
    ) -> Result<TodoEntity, sqlx::Error> {
        let result = sqlx::query("INSERT INTO todos (title, content) VALUES (?, ?)")
            .bind(title)
            .bind(content)
            .execute(&self.pool)
            .await?;

        let id = result.last_insert_id() as i64;
        self.find_one_todo_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update_todo_by_id(
        &self,
        id: i64,
        title: String,
        content: String,
    ) -> Result<TodoEntity, sqlx::Error> {
        let result = sqlx::query("UPDATE todos SET title = ?, content = ? WHERE id = ?")
            .bind(title)
            .bind(content)
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        self.find_one_todo_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete_todo_by_id(&self, id: i64) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn done_todo_by_id(&self, id: i64) -> Result<(), sqlx::Error> {
        let result = sqlx::query("UPDATE todos SET done_at = ? WHERE id = ?")
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
