mod endpoints;
mod models;
use actix_web::{web, App, HttpServer};
use endpoints::Health;

use rust_helpers::logger::init_logger;
use rust_helpers::mongo;
use rust_helpers::web_server::{self, Endpoint};
use std::io::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    init_logger();
    let mongo_client = mongo::client::create().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone()))
            .wrap(web_server::get_default_cors_middleware())
            .wrap(web_server::get_default_logger_middleware())
            .service(Health::services())
    })
    .workers(4)
    .bind(("0.0.0.0", 3005))?
    .run()
    .await
}
