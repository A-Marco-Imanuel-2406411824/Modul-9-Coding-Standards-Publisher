use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{Connection, ConnectionProperties, ExchangeKind};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to RabbitMQ
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    ).await?;

    // Create a channel
    let channel = conn.create_channel().await?;

    // Declare exchange
    let exchange_name = "user_created";
    channel.exchange_declare(
        exchange_name,
        ExchangeKind::Fanout,
        lapin::options::ExchangeDeclareOptions::default(),
        Default::default(),
    ).await?;

    // Create messages and publish them
    let messages = vec![
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "129500004y-Amir".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "129500004y-Budi".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "129500004y-Cica".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "129500004y-Dira".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "129500004y-Emir".to_owned(),
        },
    ];

    // Publish each message
    for message in messages {
        let payload = borsh::to_vec(&message)?;
        channel.basic_publish(
            exchange_name,
            "",
            lapin::options::BasicPublishOptions::default(),
            &payload,
            lapin::BasicProperties::default(),
        ).await?;
        println!("Published message: {:?}", message);
    }

    Ok(())
}
