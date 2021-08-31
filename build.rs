fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["protos/googleapis/google/pubsub/v1/pubsub.proto"],
            &["protos/googleapis"],
        )
        .unwrap();
}
