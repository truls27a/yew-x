use super::ports::UserRepository;
use crate::application::shared::unit_of_work::UnitOfWork;
use crate::domain::error::AppError;
use crate::domain::users::entities::User;

pub struct GetUser<U: UnitOfWork> {
    uow: U,
}

impl<U: UnitOfWork> GetUser<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(self, id: &str) -> Result<User, AppError> {
        self.uow.users().find_by_id(id).await?.ok_or(AppError::NotFound {
            resource_type: "User",
            field: "id",
            value: id.to_string(),
        })
    }
}
