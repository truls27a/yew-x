use super::ports::UserRepository;
use crate::domain::error::AppError;
use crate::domain::users::entities::User;

pub struct GetUser<'a, T: UserRepository> {
    repo: &'a T,
}

impl<'a, T: UserRepository> GetUser<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: &str) -> Result<User, AppError> {
        self.repo.find_by_id(id).await?.ok_or(AppError::NotFound {
            resource_type: "User",
            field: "id",
            value: id.to_string(),
        })
    }
}
