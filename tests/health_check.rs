// TEST_LOG=true cargo test health_check_works | bunyan

use once_cell::sync::Lazy;
use rci_server::{
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use reqwest::Client;
use std::net::TcpListener;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info";
    let subscriber_name = "test";
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(&subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
}

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());

    assert_eq!(Some(0), response.content_length())
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();

    let port = listener.local_addr().unwrap().port();

    let address = format!("http://0.0.0.0:{}", port);

    let server = run(listener).expect("failed to bin address");

    let _ = tokio::spawn(server);

    TestApp { address }
}
