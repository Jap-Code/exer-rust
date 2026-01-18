
use std::io;
use tracing::info;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Startup..");

    let addr = String::from("amqp://127.0.0.1:5672");

    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?; // ? führt hier zu einem Fehler..    

    info!("Connected!");

    let channel_a = conn.create_channel().await?;
    let queue = channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await;

    info!(?queue, "Declared queue");
    
    loop{
        println!("Please enter text: \n");
        let mut payload = String::new();
        io::stdin()
            .read_line(&mut payload)
            .expect("Should be able to read..");

        channel_a
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                payload.as_bytes(),
                BasicProperties::default(),
            )
            .await?;

    }
    Ok(())
}