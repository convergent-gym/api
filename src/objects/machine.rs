use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MachineStatus {
	Open,
	InUse,
	Maintence,
}

#[derive(Serialize)]
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
