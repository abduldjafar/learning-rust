use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse};
use data_sources::User;
use data_sources::{data_sources::Datasource, initialization::Initialization};

use crate::data_sources;

#[get("/api/v1/users/{user_id}")] // <- define path parameters
async fn get_user_by_id(path: web::Path<String>) -> HttpResponse {
    let db = Initialization::get("mongo");

    let user_id = path.into_inner();
    let result = db.get_user(user_id.to_string()).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/api/v1/users")]
async fn get_users() ->HttpResponse {
    let db = Initialization::get("mongo");
    let result = db.get_users().await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}
#[post("/api/v1/users")]
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
