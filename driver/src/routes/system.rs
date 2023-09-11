use std::process::Command;

use axum::http::StatusCode;
use axum::response::IntoResponse;
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