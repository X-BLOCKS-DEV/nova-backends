use crate::modules::Modules;
use crate::routes::user::{
    create_user, delete_user, get_user, update_user
};
use crate::routes::system::shutdown;
use axum::routing::{get, post};
use axum::{Extension, Router};
use dotenv::dotenv;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

pub async fn startup(modules: Arc<Modules>) {
    let system_router = Router::new()
        .route("/shutdown", get(shutdown));

    let user_router = Router::new()
        .route("/", post(create_user))
        .route(
            "/:id",
                get(get_user)
                .put(update_user)
                .delete(delete_user)
        );

    let app = Router::new()
        .nest("/v1/users", user_router)
        .nest("/v1/system", system_router)
        .layer(Extension(modules));

    let addr = SocketAddr::from(init_addr());
    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}

fn init_addr() -> (IpAddr, u16) {
    let env_host = env::var_os("HOST").expect("HOST is undefined.");
    let ip_addr = env_host
        .into_string()
        .expect("HOST is invalid.")
        .parse::<IpAddr>()
        .expect("HOST is invalid.");

    let env_port = env::var_os("PORT").expect("PORT is undefined.");
    let port = env_port
        .into_string()
        .expect("PORT is invalid.")
        .parse::<u16>()
        .expect("PORT is invalid.");

    tracing::debug!("Init ip address.");
    (ip_addr, port)
}