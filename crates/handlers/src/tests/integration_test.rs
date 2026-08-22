#[allow(unused)]
pub mod integration_tests {
    use std::{io, net::TcpListener};

    use crate::{handlers::health_check_handler, run::run};

    #[tokio::test]
    async fn health_check_succeeds() {
        let address = spawn_app();

        let client = reqwest::Client::new();

        match client.get(format!("{}/health_check", address)).send().await {
            Ok(response) => {
                assert!(response.status().is_success());
                assert_eq!(Some(0), response.content_length())
            }
            Err(why) => {
                println!("Failed to send request {why}")
            }
        }
    }

    pub fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind port");

        let port = listener.local_addr().unwrap().port();

        match run(listener) {
            Ok(server) => {
                tokio::spawn(server);
                format!("http://127.0.0.1:{}", port)
            }
            Err(why) => {
                format!("Failed to spawn server: {}", why)
            }
        }
    }
}
