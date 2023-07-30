mod users;
use actix_web::middleware::Logger;
use actix_web::{ App, HttpServer};
use env_logger::Env;
use users::{get_user_by_id, create_user,get_users};


pub async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(create_user)
            .service(get_user_by_id)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
