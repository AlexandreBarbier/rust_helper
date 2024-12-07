#[cfg(feature = "logger")]
pub mod logger;

pub mod macros;
#[cfg(feature = "mongo")]
pub mod mongo;

#[cfg(feature = "web_server")]
pub mod web_server;

#[cfg(feature = "redis")]
pub mod redis;

#[cfg(feature = "websocket")]
pub mod websocket;

#[cfg(feature = "proc")]
pub extern crate proc;

#[cfg(feature = "proc")]
pub use proc::authenticated;
