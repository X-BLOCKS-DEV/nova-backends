use std::process::Command;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use tracing::log::{ error, info };

pub async fn shutdown() -> Result<impl IntoResponse, StatusCode> {
    match Command::new("sudo").arg("/sbin/poweroff").output() {
        Ok(_) => { 
            info!("SHUTDOWN NOW ...");
            Ok(StatusCode::OK) }
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn reboot() -> Result<impl IntoResponse, StatusCode> {
    match Command::new("sudo").arg("/sbin/reboot").output() {
        Ok(_) => { 
            info!("REBOOT NOW ...");
            Ok(StatusCode::OK) }
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Serialize)]
struct NodeInfo {
    version: String,
    node_id: String,
    binary_path: String,
}

pub async fn info() -> Result<impl IntoResponse, StatusCode> {
    let json = NodeInfo {
        version: "v1.0.0".to_string(),
        node_id: "EIZEN-0000-0000".to_string(),
        binary_path: "".to_string(),
    };
    Ok((StatusCode::OK, Json(json)))
}
