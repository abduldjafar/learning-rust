pub mod data_sources;
pub mod custom_error;
pub mod api;

use std::io::Error;
use api::actix;


#[actix_web::main]
async fn main() -> Result<(), Error> {
    actix::main().await
}
