pub mod database;
pub mod models;
mod crypto;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
