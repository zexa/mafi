use axum::{response::IntoResponse, Json};
use strum::VariantNames;

use crate::roles::role::Role;

pub async fn get_all_roles() -> impl IntoResponse {
    Json(Role::VARIANTS)
}
