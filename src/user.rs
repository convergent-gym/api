enum SSOProvider { 
	GOOGLE,
	APPLE,
}

struct User {
	id: String,
	name: String,
	sso_provider: SSOProvider,
	sso_token: String
}
