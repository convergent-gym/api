enum MachineStatus {
	OPEN,
	IN_USE,
	MAINTENCE,
}

struct Machine {
	id: String,
	gym_id: String,
	type: String,
	manufacturer: String,
	model_num: String,
	max_weight: u32,
	status: MachineStatus,
	secret: String,
} 
