use rust_helpers::proc::endpoint;

use actix_web::{get, web, HttpResponse, Responder};
use rust_helpers::web_server::Endpoint;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
#[endpoint(scope="health" services="get_health")]
pub struct Health {}

#[get("")]
pub async fn get_health() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "service_name": "Api exemple",
    }))
}
