use actix_web::{get, post, web, App, HttpServer, HttpResponse, HttpRequest};
use chrono::{Utc};

mod objects;
use futures::StreamExt;
use objects::flag::{Flag, self};
pub use objects::user::{User, SSOProvider};
pub use objects::distance_record::{DistanceRecord, DistanceRecordRequest};
pub use objects::gym::{Gym};
pub use objects::gym_fav::{GymFav};
pub use objects::machine::{Machine, MachineStatus};
use std::env;
use firestore::*; 
use futures::stream::BoxStream;
use uuid::Uuid;

async fn get_flag(flagName: &str) -> String {
    let db = FirestoreDb::with_options_token_source(
        FirestoreDbOptions::new(String::from("convergentgymiot")),
        gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
        gcloud_sdk::TokenSourceType::Json(env::var("FIREBASE_KEY").expect("Expected a FIREBASE_KEY env variable"))
    ).await.unwrap();

    let object_stream: Flag = db.fluent()
    .select()
    .by_id_in("flags")
    .obj()
    .one(flagName)
    .await.unwrap().unwrap();

    return String::from(object_stream.value.clone());
}

#[post("/machines/{machine_id}/records")]
async fn create_distance_record(req: HttpRequest, info: web::Json<DistanceRecordRequest>) -> HttpResponse {
    let machine_id: String = req.match_info().get("machine_id").unwrap().parse().unwrap();

    let db = FirestoreDb::with_options_token_source(
        FirestoreDbOptions::new(String::from("convergentgymiot")),
        gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
        gcloud_sdk::TokenSourceType::Json(env::var("FIREBASE_KEY").expect("Expected a FIREBASE_KEY env variable"))
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

    let dist_threshold: f32 = get_flag("distance_threshold").await.parse().unwrap();
    
    let mut new_status: MachineStatus = MachineStatus::Open;
    if resultant_record.distance <= dist_threshold {
        new_status = MachineStatus::Taken;
    }

    let result = db.fluent()
        .update()
        .fields(paths!(Machine::{status}))
        .in_col("machines")
        .document_id(&distance_record.machine_id)
        .object(&Machine  {
            gym_id: String::from(""),
            id: String::from(distance_record.machine_id),
            kind: String::from(""),
            manufacturer: String::from(""),
            model_num: String::from(""),
            max_weight: 0,
            secret: String::from(""),
            status: new_status
        })
        .execute::<Machine>().await;

    if result.is_err() {
        println!("Error: {}", result.err().unwrap().to_string());
    }

    return HttpResponse::Ok().into();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_distance_record)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
