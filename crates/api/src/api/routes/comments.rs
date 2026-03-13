use axum::extract::{Path, State};
use axum::Json;

use crate::api::errors::ApiError;
use crate::api::middleware::{Caller, OptionalCaller};
use crate::api::schemas::{CommentResponse, CreateCommentRequest};
use crate::infrastructure::shared::unit_of_work::SqliteUnitOfWork;
use crate::AppState;

pub async fn list_comments(
    OptionalCaller(_caller): OptionalCaller,
    State(state): State<AppState>,
    Path(tweet_id): Path<String>,
) -> Result<Json<Vec<CommentResponse>>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let comments = state.get_comments_use_case.execute(uow, &tweet_id).await?;
    Ok(Json(comments.into_iter().map(CommentResponse::from).collect()))
}

pub async fn create_comment(
    caller: Caller,
    State(state): State<AppState>,
    Path(tweet_id): Path<String>,
    Json(body): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let comment = state
        .create_comment_use_case
        .execute(uow, &tweet_id, &caller.user_id, &body.content)
        .await?;
    Ok(Json(CommentResponse::from(comment)))
}
