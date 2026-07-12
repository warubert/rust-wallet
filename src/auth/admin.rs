use axum::extract::FromRequestParts;

use crate::app::AppState;
use crate::auth::user::User;
use crate::error::AppError;

pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = User::from_request_parts(parts, state).await?;
        if user.is_admin() {
            Ok(Admin)
        } else {
            Err(AppError::Forbidden)
        }
    }
}