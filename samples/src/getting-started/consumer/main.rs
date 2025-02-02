use iggy::client::Client;
use iggy::consumer_type::ConsumerType;
use iggy::messages::poll_messages::{Format, Kind, PollMessages};
use iggy::models::message::Message;
use iggy::tcp::client::TcpClient;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

const STREAM_ID: u32 = 1;
const TOPIC_ID: u32 = 1;
const PARTITION_ID: u32 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let mut client = TcpClient::new("127.0.0.1:8090")?;
    client.connect().await?;
    consume_messages(&client).await
}

async fn consume_messages(client: &dyn Client) -> Result<(), Box<dyn Error>> {
    let interval = Duration::from_millis(500);
    info!(
        "Messages will be consumed from stream: {}, topic: {}, partition: {} with interval {} ms.",
        STREAM_ID,
        TOPIC_ID,
        PARTITION_ID,
        interval.as_millis()
    );

    let mut offset = 0;
    let messages_per_batch = 10;
    loop {
        let messages = client
            .poll_messages(&PollMessages {
                consumer_type: ConsumerType::Consumer,
                consumer_id: 0,
                stream_id: STREAM_ID,
                topic_id: TOPIC_ID,
                partition_id: PARTITION_ID,
                kind: Kind::Offset,
                value: offset,
                count: messages_per_batch,
                auto_commit: false,
                format: Format::None,
            })
            .await?;
        if messages.is_empty() {
            info!("No messages found.");
            sleep(interval).await;
            continue;
        }

        offset += messages.len() as u64;
        for message in messages {
            handle_message(&message)?;
        }
        sleep(interval).await;
    }
}

fn handle_message(message: &Message) -> Result<(), Box<dyn Error>> {
    // The payload can be of any type as it is a raw byte array. In this case it's a simple string.
    let payload = std::str::from_utf8(&message.payload)?;
    info!(
        "Handling message at offset: {}, payload: {}...",
        message.offset, payload
    );
    Ok(())
}
