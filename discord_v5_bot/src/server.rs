use axum::Router;
use axum::routing::get;

pub async fn run_server() {
    let app = Router::new().route("/", get(|| async {"Hello, World!"}));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.expect("server error");
}