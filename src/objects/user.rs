use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SSOProvider { 
	Google,
	Apple,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	pub id: String,
	pub name: String,
	pub sso_provider: SSOProvider,
	pub sso_token: String
}


