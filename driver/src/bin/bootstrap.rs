use std::sync::Arc;
use nova_driver::modules::Modules;
use nova_driver::startup::{startup, init_app};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let modules = Modules::new().await;
    let _ = startup(Arc::new(modules)).await;

    Ok(())
}