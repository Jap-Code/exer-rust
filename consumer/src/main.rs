use tracing::info;
use futures_util::StreamExt;
use lapin::{
    options::*, types::FieldTable, ConnectionProperties, Connection,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting consumer..");

    let addr = String::from("amqp://127.0.0.1:5672");

    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    info!("Connected!");

    let channel_a = conn.create_channel().await?;
    let queue = channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel_a
        .basic_consume(
            "hello",
            "rust_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Waiting for messages. To quit press STRG+C!");

    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                let data = String::from_utf8_lossy(&delivery.data);
                println!("Message content: {}", data);

                delivery
                    .ack(BasicAckOptions::default())
                    .await?;
            }
            Err(error) => {
                eprintln!("Error while receiving message: {}", error);
            }
        }

    }
    Ok(())
}