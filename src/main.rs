use crate::app::App;

mod app;
pub mod models;
pub mod routes;
pub mod auth;
pub mod error;
pub mod repository;

// gera um contexto async dentro da main de forma automatica, sem precisar criar uma runtime manualmente
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}