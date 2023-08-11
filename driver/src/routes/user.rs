use crate::context::response_helper::JsonErrorResponse;
use crate::context::validate::ValidatedRequest;
use crate::models::user::{JsonCreateUser, JsonUser, JsonUpdateUser};
use crate::modules::{Modules, ModulesExt};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;
use tracing::log::{error, info};

pub async fn get_user(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules.user_use_case().get_user(id).await;

    match resp {
        Ok(uv) => uv
            .map(|uv| {
                info!("Found user `{}`.", uv.id);
                let json: JsonUser = uv.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| {
                error!("User is not found.");
                StatusCode::NOT_FOUND
            }),
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn create_user(
    ValidatedRequest(source): ValidatedRequest<JsonCreateUser>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules.user_use_case().register_user(source.into()).await;

    resp.map(|uv| {
        info!("Created user: {}", uv.id);
        let json: JsonUser = uv.into();
        (StatusCode::CREATED, Json(json))
    })
    .map_err(|err| {
        error!("{:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub async fn update_user(
    Path(id): Path<String>,
    ValidatedRequest(source): ValidatedRequest<JsonUpdateUser>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match source.validate(id) {
        Ok(usr) => {
            let resp = modules.user_use_case().update_user(usr).await;

            resp.map(|uv| {
                info!("Updated user {}", uv.id);
                let json: JsonUser = uv.into();
                (StatusCode::OK, Json(json))
            })
            .map_err(|err| {
                error!("{:?}", err);

                if err.to_string() == *"`statusCode` is invalid." {
                    let json = JsonErrorResponse::new(
                        "invalid_request".to_string(),
                        vec![err.to_string()],
                    );
                    (StatusCode::BAD_REQUEST, Json(json))
                } else {
                    let json = JsonErrorResponse::new(
                        "server_error".to_string(),
                        vec!["INTERNAL SERVER ERROR".to_string()],
                    );
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json))
                }
            })
        }
        Err(errors) => {
            let json = JsonErrorResponse::new("invalid_request".to_string(), errors);
            Err((StatusCode::BAD_REQUEST, Json(json)))
        }
    }
}

pub async fn delete_user(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules.user_use_case().delete_user(id).await;

    match resp {
        Ok(uv) => uv
            .map(|uv| {
                info!("Deleted user `{}`.", uv.id);
                let json: JsonUser = uv.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| {
                error!("User is not found.");
                StatusCode::NOT_FOUND
            }),
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}