use serde::Serialize;

#[derive(Serialize)]
pub enum SSOProvider { 
	GOOGLE,
	APPLE,
}

#[derive(Serialize)]
pub struct User {
	pub id: String,
	pub name: String,
	pub sso_provider: SSOProvider,
	pub sso_token: String
}


