use crate::application::shared::time::Clock;

pub struct UtcClock;

impl Clock for UtcClock {
    fn now(&self) -> i64 {
        chrono::Utc::now().timestamp()
    }
}
