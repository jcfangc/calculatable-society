mod _config;
mod components;
mod events;
mod models;

use _config::database::database_config;
use _config::log::log_config;

#[tokio::main]
async fn main() {
    log_config::init_logging();
    database_config::create_pool(Some(30), Some(15)).await;
}
