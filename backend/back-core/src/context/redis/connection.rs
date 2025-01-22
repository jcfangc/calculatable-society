use redis::AsyncCommands;
use tokio;

use redis::AsyncCommands;
use tokio;

pub(crate) async fn init_redis_connection(
    host: Option<&str>,
    port: Option<&str>,
    password: Option<&str>,
) -> redis::RedisResult<redis::aio::Connection> {
    // 使用外部传入的参数，如果未传入则从环境变量获取，最终使用默认值
    let redis_host = host
        .map(|h| h.to_string())
        .or_else(|| std::env::var("REDIS_HOST").ok())
        .unwrap_or_else(|| "localhost".to_string());

    let redis_port = port
        .map(|p| p.to_string())
        .or_else(|| std::env::var("REDIS_PORT").ok())
        .unwrap_or_else(|| "6379".to_string());

    let redis_password = password
        .map(|pwd| pwd.to_string())
        .or_else(|| std::env::var("REDIS_PASSWORD").ok())
        .unwrap_or_else(|| "".to_string());

    // 动态组合 Redis URL，包含密码
    let redis_url = if redis_password.is_empty() {
        // 如果密码为空，则不添加密码部分
        format!("redis://{}:{}", redis_host, redis_port)
    } else {
        // 如果密码不为空，则添加密码部分
        format!("redis://:{}@{}:{}", redis_password, redis_host, redis_port)
    };

    // 建立异步连接
    let client = redis::Client::open(redis_url)?;
    let conn = client.get_async_connection().await?;
    Ok(conn)
}
