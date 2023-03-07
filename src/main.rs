use actix_web::{get, web, App, Responder};
use chrono::{Utc};

mod objects;
pub use objects::user::{User, SSOProvider};
pub use objects::distance_record::{DistanceRecord};
pub use objects::gym::{Gym};
pub use objects::gym_fav::{GymFav};
pub use objects::machine::{Machine, MachineStatus};

#[get("/sample-user")]
async fn sample_user() -> web::Json<User> {
    let user_test = User {
        id: String::from("adbee93e-0dff-45a2-a958-3e5c7fda6b76"),
        sso_provider: SSOProvider::APPLE,
        sso_token: String::from("d9d9d1ba-d61d-43bf-958d-0fde71bad9b9"),
        name: String::from("testname")
    };

    web::Json(user_test)
}

#[get("/sample-distrecord")]
async fn sample_distrecord() -> web::Json<DistanceRecord> {
    let distrecord_test = DistanceRecord {
        machine_id: String::from("adbee93e-0dff-45a2-a958-3e5c7fda6b76"),
        datetime: Utc::now(),
        distance: 0.2
    };

    web::Json(distrecord_test)
}

#[get("/sample-gym")]
async fn sample_gym() -> web::Json<Gym> {
    let gym_test = Gym {
        id: String::from("adbee93e-0dff-45a2-a958-3e5c7fda6b76"),
        name: String::from("Market Street Gym"),
        lat: 39.9517176,
        lng: -75.1609552375557
    };

    web::Json(gym_test)
}

#[get("/sample-gymfav")]
async fn sample_gymfav() -> web::Json<GymFav> {
    let gymfav_test = GymFav {
        gym_id: String::from("adbee93e-0dff-45a2-a958-3e5c7fda6b76"),
        user_id: String::from("adbee93e-0dff-45a2-a958-3e5c7fda6b76"),
    };

    web::Json(gymfav_test)
}

#[get("/sample-machine")]
async fn sample_machine() -> web::Json<Machine> {
    let machine_test = Machine {
        id: String::from("adbee93e-0dff-45a2-a958-3e5c7fda6b76"),
        gym_id: String::from("adbee93e-0dff-45a2-a958-3e5c7fda6b76"),
        kind: String::from("cable_generic"),
        manufacturer: String::from("Matrix"),
        max_weight: 245,
        model_num: String::from("ABC-123"),
        status: MachineStatus::OPEN,
        secret: String::from("secret-token-here")
    };

    web::Json(machine_test)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(sample_distrecord)
            .service(sample_user)
            .service(sample_gym)
            .service(sample_gymfav)
            .service(sample_machine)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
