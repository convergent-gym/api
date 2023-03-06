use serde::Serialize;

#[derive(Serialize)]
pub enum MachineStatus {
	OPEN,
	IN_USE,
	MAINTENCE,
}

#[derive(Serialize)]
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
