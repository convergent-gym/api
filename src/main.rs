use actix_web::{get, post, web, App, HttpServer, HttpResponse, HttpRequest};
use chrono::{Utc};

mod objects;
use futures::StreamExt;
pub use objects::user::{User, SSOProvider};
pub use objects::distance_record::{DistanceRecord, DistanceRecordRequest};
pub use objects::gym::{Gym};
pub use objects::gym_fav::{GymFav};
pub use objects::machine::{Machine, MachineStatus};
use firestore::*; 
use futures::stream::BoxStream;
use uuid::Uuid;

#[post("/machines/{machine_id}/records")]
async fn create_distance_record(req: HttpRequest, info: web::Json<DistanceRecordRequest>) -> HttpResponse {
    let machine_id: String = req.match_info().get("machine_id").unwrap().parse().unwrap();

    let db = FirestoreDb::with_options_token_source(
        FirestoreDbOptions::new(String::from("convergentgymiot")),
        gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
        gcloud_sdk::TokenSourceType::File("./target/debug/key.json".into())
    ).await.unwrap();

    if info.machine_id != machine_id {
        return HttpResponse::BadRequest().into();
    }

    let distance_record: DistanceRecord = DistanceRecord {
        machine_id: machine_id,
        datetime: Utc::now(),
        distance: info.distance
    };

    let resultant_record = db.fluent()
        .insert()
        .into("distance_records")
        .document_id(Uuid::new_v4().to_string())
        .object(&distance_record)
        .execute::<DistanceRecord>()
        .await.unwrap();

    return HttpResponse::Ok().into();
}

#[get("/machines/{machine_id}/distance")]
async fn get_distance(req: HttpRequest) -> web::Json<DistanceRecord> {
    let machine_id: String = req.match_info().get("machine_id").unwrap().parse().unwrap();

    let db = FirestoreDb::with_options_token_source(
        FirestoreDbOptions::new(String::from("convergentgymiot")),
        gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
        gcloud_sdk::TokenSourceType::File("./target/debug/key.json".into())
    ).await.unwrap();

   
    let mut object_stream: BoxStream<DistanceRecord> = db.fluent()
    .select()
    .from("distance_records")
    .order_by([(
        "datetime",
        FirestoreQueryDirection::Descending,
    )])
   
    .obj() // Reading documents as structures using Serde gRPC deserializer
    .stream_query()
    .await.unwrap();

    let result: DistanceRecord = object_stream.next().await.unwrap();

    web::Json(result)
}

#[get("/sample-user")]

async fn sample_user() -> web::Json<User> {
    let user_test = User {
        id: String::from("adbee93e-0dff-45a2-a958-3e5c7fda6b76"),
        sso_provider: SSOProvider::Apple,
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
        status: MachineStatus::Open,
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
            .service(create_distance_record)
            .service(get_distance)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
