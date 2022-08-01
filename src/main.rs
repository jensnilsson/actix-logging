use actix_web::{web, post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use log::info;
use log4rs;

#[derive(Debug)]
struct UserTransaction {
    payload: String,
    time_received: DateTime<Utc>,
}

#[post("/test")]
async fn test(payload: web::Json<serde_json::Value>) -> impl Responder {
    let now: DateTime<Utc> = Utc::now();
    let payload: String = payload.to_string();
    let user_transaction = UserTransaction {
        time_received: now,
        payload
    };
    info!("Payload, {:?}", &user_transaction);
    HttpResponse::Ok().body(user_transaction.payload)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    info!("Hello {:?}", &name);
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting API Gateway... :)");
    println!["Starting API Gateway... :)"];
    HttpServer::new(|| {
        App::new()
            .service(test)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
