use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum MachineStatus {
	Open,
	Taken,
	Maintence,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
	pub id: String,
	pub gym_id: String,
	pub kind: String,
	pub manufacturer: String,
	pub model_num: String,
	pub max_weight: u32,
	pub status: MachineStatus,
	pub secret: String,
} 
