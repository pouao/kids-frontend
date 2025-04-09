use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use kids_frontend::util::constant::CFG;
use kids_frontend::routers::push_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    format!(
                        "{}=debug, tower_http=debug",
                        env!("CARGO_CRATE_NAME")
                    )
                    .into()
                }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_router = push_router().await;

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        CFG.get("HOST").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await
    .unwrap();
    tracing::info!(
        "kids web-server: http://{}",
        listener.local_addr().unwrap(),
    );
    axum::serve(listener, app_router).await.unwrap()
}
