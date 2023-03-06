use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct DistanceRecord {
	pub machine_id: String,
	pub datetime: DateTime<Utc>,
	pub distance: f32,
}
