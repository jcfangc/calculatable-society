use database::database_config::create_pool;

#[tokio::main]
async fn main() {
    create_pool(Some(30), Some(15)).await;
}
