use actix_web::{get, post, web, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::http::header::ContentType;
use chrono::{Utc};

mod objects;
pub use objects::user::{User, SSOProvider};
pub use objects::distance_record::{DistanceRecord, DistanceRecordRequest};
pub use objects::gym::{Gym};
pub use objects::gym_fav::{GymFav};
pub use objects::machine::{Machine, MachineStatus};
use tokio_postgres::*;
use std::env;

// TODO: ths is definitely not the proper way to handle connections
async fn connect_to_db() -> tokio_postgres::Client {
    let db_uri_env =  env::var("DB_URI").expect("Expected Database URI");
    let db_uri =  db_uri_env.as_str();

    let (client, connection) = tokio_postgres::connect(db_uri, NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    return client;
}

#[post("/machines/{machine_id}/records")]
async fn create_distance_record(req: HttpRequest, info: web::Json<DistanceRecordRequest>) -> HttpResponse {
    let machine_id: String = req.match_info().get("machine_id").unwrap().parse().unwrap();

    if info.machine_id != machine_id {
        return HttpResponse::BadRequest().into();
    }

    let distance_record: DistanceRecord = DistanceRecord {
        machine_id: machine_id,
        datetime: Utc::now(),
        distance: info.distance
    };

    let client = connect_to_db().await;

    let result = client
        .query("INSERT INTO distance_records (machine_id, datetime, distance) VALUES($1, $2, $3);", &[ &distance_record.machine_id, &distance_record.datetime, &distance_record.distance])
        .await;

    if result.is_err() {
        return HttpResponse::InternalServerError().into();
    }

    return HttpResponse::Ok().into();
}

#[get("/machines/{machine_id}/distance")]
async fn get_distance(req: HttpRequest) -> HttpResponse {
    let machine_id: String = req.match_info().get("machine_id").unwrap().parse().unwrap();

    let client = connect_to_db().await;

    let result = client
        .query("SELECT machine_id, datetime, distance FROM distance_records WHERE machine_id = $1 ORDER BY datetime DESC LIMIT 1;", &[ &machine_id ])
        .await;

    if result.is_err() {
        return HttpResponse::InternalServerError().into();
    }

    let result_record = result.unwrap();

    let result_object: DistanceRecord = DistanceRecord { machine_id:  result_record[0].get(0), datetime:  result_record[0].get(1), distance:  result_record[0].get(2) };

    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(web::Json(result_object)).into()
}

// Below return sample objects

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
