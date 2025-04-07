


use kids_frontend::util::constant::CFG;
use kids_frontend::routers::push_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    
    let app_router = push_router().await;

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        CFG.get("HOST").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await
    .unwrap();
    tracing::info!(
        "odds web-server: http://{}",
        listener.local_addr().unwrap(),
    );
    axum::serve(listener, app_router).await.unwrap()
}


