use std::net::TcpListener;
use zero2prod::configuration::get_config();
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_config().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:8000", configuration.application.port);
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    println!("Listening on port {}", port);

    run(listener)?.await
}
