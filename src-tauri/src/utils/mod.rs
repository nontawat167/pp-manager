pub fn generate_uuid() -> String {
    use uuid::Uuid;
    let id = Uuid::new_v4().to_string();
    id
}

pub fn get_timestamp() -> String {
    use chrono::{FixedOffset, Utc};
    let hour = 3600;
    let timezone = FixedOffset::east_opt(7 * hour).unwrap();
    let time = Utc::now()
        .with_timezone(&timezone)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    time
}
