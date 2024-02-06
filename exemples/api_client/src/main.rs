mod endpoints;
use std::io::Error;

use actix_web::{App, HttpServer};
use endpoints::Health;
use rust_helpers::web_server::{self, Endpoint};

#[actix_web::main]
async fn main() -> Result<(), Error> {
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
