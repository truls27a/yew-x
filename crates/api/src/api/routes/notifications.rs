use axum::extract::State;
use axum::Json;

use crate::api::errors::AppError;
use crate::api::middleware::Caller;
use crate::api::schemas::NotificationResponse;
use crate::application::notifications::use_cases;
use crate::AppState;

pub async fn list_notifications(
    caller: Caller,
    State(state): State<AppState>,
) -> Result<Json<Vec<NotificationResponse>>, AppError> {
    let uc = use_cases::GetNotifications::new(&state.notification_repo);
    let notifications = uc.execute(&caller.user_id).await?;
    Ok(Json(
        notifications
            .into_iter()
            .map(NotificationResponse::from)
            .collect(),
    ))
}
