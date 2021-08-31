pub mod api {
    tonic::include_proto!("google.pubsub.v1");
}

pub use api::{
    publisher_client::PublisherClient, ListTopicsRequest, PublishRequest, PubsubMessage,
};
