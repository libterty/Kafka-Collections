use futures::*;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::message::{OwnedHeaders, Headers};
use rdkafka::{ClientContext, TopicPartitionList, Message};
use rdkafka::consumer::{ConsumerContext, Rebalance, StreamConsumer, Consumer, CommitMode};
use rdkafka::error::KafkaResult;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        println!("Commiting offsets {:?}", result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
    
}