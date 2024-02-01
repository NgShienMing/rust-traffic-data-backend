use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Traffic {
    pub id: i32,
    pub zone: String,
    pub district: String,
    pub device: String,
    pub object_count: i32,
    pub vehicle_count: i32,
    pub animal_count: i32,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}
