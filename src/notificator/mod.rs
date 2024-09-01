pub trait Notificator: Send + Sync {
    fn notify(&self, key: String, value: String);
}

#[derive(Default)]
pub struct ConsoleNotificator;

impl Notificator for ConsoleNotificator {
    fn notify(&self, key: String, value: String) {
        println!("{}: {}", key, value);
    }
}
