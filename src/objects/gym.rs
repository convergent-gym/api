use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Gym {
	pub id: String,
	pub name: String,
	pub lat: f32,
	pub lng: f32,
}
