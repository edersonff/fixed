use std::sync::Mutex;

// HOME is process-wide state; tests across different modules that redirect it to a temp
// directory run on separate threads under `cargo test` and corrupt each other without this lock.
pub(crate) static HOME_ENV_LOCK: Mutex<()> = Mutex::new(());
