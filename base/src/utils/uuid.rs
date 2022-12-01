use uuid::Uuid;

pub fn short_uuid() -> String {
    Uuid::new_v4().as_simple().to_string()
}

pub fn request_uuid() -> String {
    format!("Rid{}", Uuid::new_v4().as_simple().to_string())
}
