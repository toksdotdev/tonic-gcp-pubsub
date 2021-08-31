mod google;

use std::collections::HashMap;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    IntoRequest,
};

use self::google::{PublishRequest, PublisherClient, PubsubMessage};

pub(crate) const CERTS: &[u8] = include_bytes!("../certs.pem");
pub(crate) const DOMAIN: &'static str = "pubsub.googleapis.com";
pub(crate) const ENDPOINT: &'static str = "https://pubsub.googleapis.com";

#[tokio::main]
async fn main() {
    let api_token = std::env::var("GCP_AUTH_TOKEN").unwrap();
    let project = std::env::var("GCP_PROJECT").unwrap();
    let topic_name = std::env::var("GCP_TOPIC_NAME").unwrap();
    let topic = format!("projects/{}/topics/{}", project, topic_name);

    let sample_message = "hello world".as_bytes();
    let bearer_token = format!("Bearer {}", api_token);
    let header_value = MetadataValue::from_str(&bearer_token).unwrap();

    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTS))
        .domain_name(DOMAIN);

    let channel = Channel::from_static(ENDPOINT)
        .tls_config(tls_config)
        .unwrap()
        .connect()
        .await
        .unwrap();

    let mut publisher = PublisherClient::new(channel.clone());
    let mut request = PublishRequest {
        topic: topic.to_string(),
        messages: vec![PubsubMessage {
            data: sample_message.into(),
            attributes: HashMap::new(),
            message_id: String::new(),
            ordering_key: String::new(),
            publish_time: None,
        }],
    }
    .into_request();

    let metadata = request.metadata_mut();
    metadata.insert("authorization", header_value.clone());
    let response = publisher.publish(request).await.unwrap();

    println!("{:?}", response);
}
