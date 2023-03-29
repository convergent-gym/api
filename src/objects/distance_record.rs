use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistanceRecord {
	pub machine_id: String,
	#[serde(with = "firestore::serialize_as_timestamp")]
	pub datetime: DateTime<Utc>,
	pub distance: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistanceRecordRequest {
	pub machine_id: String,
	pub auth_key: String,
	pub distance: f32,
}
