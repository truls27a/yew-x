use crate::application::ports::user_repository::UserRepository;
use crate::domain::entities::user::User;

pub struct GetUser<'a, T: UserRepository> {
    repo: &'a T,
}

impl<'a, T: UserRepository> GetUser<'a, T> {
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: &str) -> anyhow::Result<Option<User>> {
        self.repo.find_by_id(id).await
    }
}
