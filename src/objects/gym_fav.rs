use serde::Serialize;

#[derive(Serialize)]
pub struct GymFav {
	pub user_id: String,
	pub gym_id: String,
}
