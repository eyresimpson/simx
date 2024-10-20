use uuid::Uuid;

pub fn getUuid() -> String {
    Uuid::new_v4().to_string()
}