use std::net::TcpListener;
use rust_subscriber::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
       .await
       .expect("Failed to connect to Postgres.");

    let address = format!("127.0.01:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}