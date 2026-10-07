use crate::error::AppError;
use crate::{model::TodoResponse, service::TodoService};
use axum::http::StatusCode;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub content: String,
}

pub fn routes(todo_service: TodoService) -> Router {
    Router::new()
        .route("/api/v1/todos", get(get_all_todos).post(create_todo))
        .route(
            "/api/v1/todos/{id}",
            get(get_one_todo)
                .put(update_todo)
                .delete(delete_todo)
                .patch(done_todo),
        )
        .with_state(todo_service)
}

async fn get_all_todos(
    State(service): State<TodoService>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    Ok(Json(service.get_all_todos().await?))
}

async fn get_one_todo(
    State(service): State<TodoService>,
    Path(id): Path<i64>,
) -> Result<Json<Option<TodoResponse>>, AppError> {
    let todo = service.get_one_todo(id).await?.ok_or(AppError::NotFound)?;
    Ok(Json(Some(todo)))
}

async fn create_todo(
    State(service): State<TodoService>,
    Json(request): Json<CreateTodoRequest>,
) -> Result<Json<TodoResponse>, AppError> {
    Ok(Json(
        service.create_todo(request.title, request.content).await?,
    ))
}

async fn update_todo(
    State(service): State<TodoService>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, AppError> {
    Ok(Json(
        service
            .update_todo(id, request.title, request.content)
            .await?,
    ))
}

async fn delete_todo(
    State(service): State<TodoService>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    service.delete_todo(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn done_todo(
    State(service): State<TodoService>,
    Path(id): Path<i64>,
) -> Result<Json<()>, AppError> {
    Ok(Json(service.done_todo(id).await?))
}
