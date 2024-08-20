mod metrika;
mod server;
mod utils;

use server::server::server_router;
use tokio;

#[tokio::main]
async fn main() {
    let _ = server_router().await;
}
