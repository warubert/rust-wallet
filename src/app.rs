use axum::Router;
use tokio::{net::TcpListener, sync::Mutex};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt
};

use crate::models::Asset;
use crate::routes;

#[derive(Clone)]
pub struct AppState {
    // para compartilhar o mesmo estado(ref para o mesmo vetor) entre as rotas
    // mutex -> acesso unico mutavel
    pub assets: Arc<Mutex<Vec<Asset>>>
}

impl AppState {
    fn new() -> Self {
        Self {
            assets: Default::default()
        }
    }
}

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        // para logs
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();
        tracing_subscriber::registry().with(layer).init();

        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        let router = Router::new()
            .nest("/api", routes::api::router())
            .with_state(AppState::new());

        info!("Server started at http://localhost:3000");

        axum::serve(listener, router).await?;

        Ok(())
    }
}