use axum::extract::State;
use axum::Json;

use crate::api::errors::ApiError;
use crate::api::middleware::Caller;
use crate::api::schemas::NotificationResponse;
use crate::application::notifications::use_cases;
use crate::infrastructure::shared::unit_of_work::SqliteUnitOfWork;
use crate::AppState;

pub async fn list_notifications(
    caller: Caller,
    State(state): State<AppState>,
) -> Result<Json<Vec<NotificationResponse>>, ApiError> {
    let uow = SqliteUnitOfWork::new(&state.db).await?;
    let notifications = use_cases::GetNotifications::new(uow).execute(&caller.user_id).await?;
    Ok(Json(
        notifications
            .into_iter()
            .map(NotificationResponse::from)
            .collect(),
    ))
}
