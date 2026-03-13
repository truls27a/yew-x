use axum::extract::State;
use axum::Json;

use crate::api::errors::AppError;
use crate::api::schemas::NotificationResponse;
use crate::application::use_cases::get_notifications;
use crate::AppState;

pub async fn list_notifications(
    State(state): State<AppState>,
) -> Result<Json<Vec<NotificationResponse>>, AppError> {
    let uc = get_notifications::GetNotifications::new(&state.notification_repo);
    let notifications = uc.execute("current").await?;
    Ok(Json(
        notifications
            .into_iter()
            .map(NotificationResponse::from)
            .collect(),
    ))
}
