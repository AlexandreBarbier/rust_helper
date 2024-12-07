mod endpoints;
mod models;
use actix_web::{App, HttpServer};
use endpoints::Health;
use models::user::User;
use rust_helpers::mongo;
use rust_helpers::web_server::{self, Endpoint};
use std::io::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let mongo_client = mongo::client::create().await;
    let usr = User::get_or_create("name".to_string(), "email".to_string(), &mongo_client).await;

    HttpServer::new(move || {
        App::new()
            .wrap(web_server::get_default_cors_middelware())
            .service(Health::services())
    })
    .workers(4)
    .bind(("0.0.0.0", 3005))?
    .run()
    .await
}
