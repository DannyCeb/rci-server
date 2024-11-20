use std::net::TcpListener;

use rci_server::{
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod", "info", std::io::stdout);

    init_subscriber(subscriber);
    let address = format!("0.0.0.0:{}", 8080);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
