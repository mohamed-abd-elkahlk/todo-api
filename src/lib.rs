use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_expiration_time(expiry_duration: u64) -> usize {
    // Return a timestamp for expiration, e.g., 1 hour from now
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (now.as_secs() + expiry_duration) as usize
}
// Function to get the current timestamp
pub fn get_current_timestamp() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
}
