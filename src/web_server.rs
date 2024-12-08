#[cfg(feature = "logger")]
use crate::logger::get_log_format;
#[cfg(feature = "logger")]
use actix_web::middleware::Logger;

use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::Scope;
use serde::{Deserialize, Serialize};

/// Trait representing an endpoint with associated services.
pub trait Endpoint {
    fn services() -> Scope;
}

/// Trait for handling identity IDs.
pub trait IdentityID {
    /// Creates an ID from user details.
    fn create_id(user_id: String, user_email: String, token: Option<String>) -> String {
        format!("{} {} {}", user_id, user_email, token.unwrap_or_default())
    }

    /// Parses the ID into its components.
    fn parse_id(&self) -> (String, String, Option<String>);
}

/// Struct representing a JSON response.
#[derive(Serialize, Deserialize)]
pub struct JsonResponse {
    pub status: i16,
    pub message: &'static str,
}

/// Trait for converting a type to a JSON response.
pub trait Response {
    fn to_response(self) -> Json<Self>
    where
        Self: std::marker::Sized;
}

/// Returns the default CORS middleware configuration.
pub fn get_default_cors_middleware() -> Cors {
    Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "HEAD", "OPTIONS"])
        .max_age(14400)
}

#[cfg(feature = "logger")]
/// Returns the default logger middleware configuration.
pub fn get_default_logger_middleware() -> Logger {
    Logger::new(get_log_format())
}
