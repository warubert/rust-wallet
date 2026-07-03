use axum::{Json, Router, extract::State, routing::get};
use serde::Deserialize;
use std::collections::HashMap;

use crate::auth::admin::Admin;
use crate::error::AppError;
use crate::{app::AppState, models::Asset};

pub fn router() -> Router<AppState> {
    Router::new().route("/assets", 
    get(list_assets)
    .post(create_asset)
    .patch(update_asset))
}

#[tracing::instrument(skip_all)]
async fn list_assets(state: State<AppState>) -> Json<HashMap<i64,Asset>> {
    let assets = state.assets.lock().await;
    Json(assets.clone())
}

#[derive(Deserialize)]
struct CreateAssetRequest {
    pub name: String,
    pub unit_value: f64,
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    _: Admin,
    state: State<AppState>, 
    Json(request): Json<CreateAssetRequest>
) -> Json<Asset> {
    let mut assets = state.assets.lock().await;

    let id = assets.values().map(|asset| asset.id).max().unwrap_or_default() + 1;

    let asset = Asset {
        id,
        name: request.name,
        unit_value: request.unit_value,
    };

    assets.insert(id, asset.clone());

    Json(asset)
}

#[derive(Deserialize)]
struct UpdateAssetRequest {
    id: i64,
    name: Option<String>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    _: Admin,
    state: State<AppState>, 
    Json(request): Json<UpdateAssetRequest>
) -> Result<Json<Asset>, AppError> {
    let mut assets = state.assets.lock().await;

    let Some(existing_asset) = assets.get_mut(&request.id) else {
        return Err(AppError::AssetNotFound);
    };

    if let Some(name) = request.name {
        existing_asset.name = name;
    }

    if let Some(unit_value) = request.unit_value {
        existing_asset.unit_value = unit_value;
    }

    Ok(Json(existing_asset.clone()))
}