mod bot;

#[tokio::main]
async fn main() {
    bot::run_bot().await
}
