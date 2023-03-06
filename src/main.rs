use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod objects;
pub use objects::user::{User, SSOProvider};

#[get("/")]
async fn hello() -> web::Json<User> {
    let user_test = User {
        id: String::from("adbee93e-0dff-45a2-a958-3e5c7fda6b76"),
        sso_provider: SSOProvider::APPLE,
        sso_token: String::from("d9d9d1ba-d61d-43bf-958d-0fde71bad9b9"),
        name: String::from("testname")
    };

    web::Json(user_test)
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
