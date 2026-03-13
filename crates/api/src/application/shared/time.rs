pub trait Clock: Send + Sync {
    fn now(&self) -> i64; // unix epoch seconds
}
