use crate::app::App;

mod app;
pub mod models;

// gera um contexto async dentro da main de forma automatica, sem precisar criar uma runtime manualmente
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}