use tokio::join;

mod bot;
mod server;

#[tokio::main]
async fn main() {
    let (_, _) = join!(
        bot::run_bot(),
        server::run_server()
    );
}
