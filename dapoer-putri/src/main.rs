pub mod data_sources;
pub mod custom_error;

use actix_web::middleware::Logger;
use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use data_sources::User;
use data_sources::{data_sources::Datasource, initialization::Initialization};
use env_logger::Env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users/{user_id}")] // <- define path parameters
async fn index(path: web::Path<String>) -> HttpResponse {
    let db = Initialization::get("mongo");

    let user_id = path.into_inner();
    let result = db.get_user(user_id.to_string()).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/users")]
pub async fn create_user(new_user: Json<User>) -> HttpResponse {
    let db = Initialization::get("mongo");

    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        address: new_user.address.to_owned(),
        email: new_user.email.to_owned(),
        gender: new_user.gender.to_owned(),
    };

    let user_detail = db.add_user(data).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(create_user)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
