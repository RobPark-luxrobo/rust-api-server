use crate::error::AppError;
use crate::model::{TodoEntity, TodoResponse};
use crate::repository::TodoRepository;

#[derive(Clone)]
pub struct TodoService {
    repository: TodoRepository,
}

impl TodoService {
    pub fn new(repository: TodoRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all_todos(&self) -> Result<Vec<TodoResponse>, AppError> {
        let result = self.repository.find_all_todos().await?;
        Ok(result.iter().map(|t| t.to_response()).collect())
    }

    pub async fn get_one_todo(&self, id: i64) -> Result<Option<TodoResponse>, AppError> {
        let result = self.repository.find_one_todo_by_id(id).await?;
        match result {
            Some(todo) => Ok(Some(todo.to_response())),
            None => Ok(None),
        }
    }

    pub async fn create_todo(
        &self,
        title: String,
        content: String,
    ) -> Result<TodoEntity, AppError> {
        let result = self.repository.create_todo(title, content).await?;
        Ok(result)
    }

    pub async fn update_todo(
        &self,
        id: i64,
        title: String,
        content: String,
    ) -> Result<TodoEntity, AppError> {
        let result = self
            .repository
            .update_todo_by_id(id, title, content)
            .await?;
        Ok(result)
    }

    pub async fn delete_todo(&self, id: i64) -> Result<(), AppError> {
        let result = self.repository.delete_todo_by_id(id).await?;
        Ok(result)
    }

    pub async fn done_todo(&self, id: i64) -> Result<(), AppError> {
        let result = self.repository.done_todo_by_id(id).await?;
        Ok(result)
    }
}
