use rust_helpers::proc::endpoint;

use crate::models::user::User;
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use rust_helpers::mongo::mongodb;
use rust_helpers::web_server::Endpoint;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Serialize, Deserialize)]
#[endpoint(scope="health" services="get_health")]
pub struct Health {}

#[get("")]
pub async fn get_health(client: Data<mongodb::Client>) -> impl Responder {
    let usr = User::get_or_create("name".to_string(), "email".to_string(), &client).await;
    println!("user {:?}", usr);
    HttpResponse::Ok().json(json!({
        "service_name": "Api exemple",
        "user": usr
    }))
}
