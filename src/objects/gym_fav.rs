use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GymFav {
	pub user_id: String,
	pub gym_id: String,
}
