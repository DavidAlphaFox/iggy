use crate::common::TestSetup;
use iggy::messages::poll_messages::Kind;
use iggy::messages::send_messages;
use iggy::messages::send_messages::Key;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use streaming::message::Message;
use streaming::polling_consumer::PollingConsumer;
use streaming::storage::SystemStorage;
use streaming::topics::topic::Topic;
use streaming::utils::hash;

mod common;

#[tokio::test]
async fn given_key_none_messages_should_be_appended_to_the_next_partition_using_round_robin() {
    let setup = TestSetup::init().await;
    let partitions_count = 3;
    let messages_per_partition_count = 10;
    let topic = init_topic(&setup, partitions_count).await;
    let key = Key::none();
    for i in 1..=partitions_count * messages_per_partition_count {
        let payload = get_payload(i);
        topic
            .append_messages(&key, vec![get_message(&payload)])
            .await
            .unwrap();
    }
    for i in 1..=partitions_count {
        assert_messages(&topic, i, messages_per_partition_count).await;
    }
}

#[tokio::test]
async fn given_key_partition_id_messages_should_be_appended_to_the_chosen_partition() {
    let setup = TestSetup::init().await;
    let partition_id = 1;
    let partitions_count = 3;
    let messages_per_partition_count = 10;
    let topic = init_topic(&setup, partitions_count).await;
    let key = Key::partition_id(partition_id);
    for i in 1..=partitions_count * messages_per_partition_count {
        let payload = get_payload(i);
        topic
            .append_messages(&key, vec![get_message(&payload)])
            .await
            .unwrap();
    }

    for i in 1..=partitions_count {
        if i == partition_id {
            assert_messages(&topic, i, messages_per_partition_count * partitions_count).await;
            continue;
        }
        assert_messages(&topic, i, 0).await;
    }
}

#[tokio::test]
async fn given_key_entity_id_messages_should_be_appended_to_the_calculated_partition() {
    let setup = TestSetup::init().await;
    let partitions_count = 3;
    let messages_count = 10;
    let topic = init_topic(&setup, partitions_count).await;
    for entity_id in 1..=partitions_count * messages_count {
        let payload = get_payload(entity_id);
        let key = Key::entity_id_u32(entity_id);
        topic
            .append_messages(&key, vec![get_message(&payload)])
            .await
            .unwrap();
    }

    let mut messages_count_per_partition = HashMap::new();
    for entity_id in 1..=partitions_count * messages_count {
        let key = Key::entity_id_u32(entity_id);
        let hash = hash::calculate(&key.value);
        let mut partition_id = (hash % partitions_count as u128) as u32;
        if partition_id == 0 {
            partition_id = partitions_count;
        }

        messages_count_per_partition
            .entry(partition_id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for (partition_id, expected_messages) in messages_count_per_partition {
        assert_messages(&topic, partition_id, expected_messages).await;
    }
}

fn get_payload(id: u32) -> String {
    format!("message-{}", id)
}

async fn assert_messages(topic: &Topic, partition_id: u32, expected_messages: u32) {
    let consumer = PollingConsumer::Consumer(0);
    let messages = topic
        .get_messages(consumer, partition_id, Kind::Offset, 0, 1000)
        .await
        .unwrap();
    assert_eq!(messages.len() as u32, expected_messages);
}

async fn init_topic(setup: &TestSetup, partitions_count: u32) -> Topic {
    let storage = Arc::new(SystemStorage::default());
    let stream_id = 1;
    let id = 2;
    let name = "test";
    let topic = Topic::create(
        stream_id,
        id,
        name,
        partitions_count,
        &setup.path,
        setup.config.stream.topic.clone(),
        storage.clone(),
    );
    topic.persist().await.unwrap();
    topic
}

fn get_message(payload: &str) -> Message {
    Message::from_message(&send_messages::Message::from_str(payload).unwrap())
}
