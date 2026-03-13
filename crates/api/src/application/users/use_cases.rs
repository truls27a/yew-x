use super::ports::UserRepository;
use crate::application::shared::unit_of_work::UnitOfWork;
use crate::domain::error::AppError;
use crate::domain::users::entities::User;

#[derive(Clone)]
pub struct GetUser;

impl GetUser {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute<U: UnitOfWork>(&self, uow: U, id: &str) -> Result<User, AppError> {
        uow.users().find_by_id(id).await?.ok_or(AppError::NotFound {
            resource_type: "User",
            field: "id",
            value: id.to_string(),
        })
    }
}
