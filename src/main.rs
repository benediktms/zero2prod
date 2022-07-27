use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    println!("Listening on port {}", port);

    run(listener)?.await
}
