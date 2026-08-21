#[allow(unused)]
#[cfg(test)]
mod integration_tests {
    use std::io;

    use crate::{ health_checker::health_check_handler, run::run };

    #[tokio::test]
    async fn health_check_succeeds() {
        spawn_app();

        let client = reqwest::Client::new();
        const URL: &str = "http://127.0.0.1:8000/health_check";

        match client.get(URL).send().await {
            Ok(response) => {
                assert!(response.status().is_success());
                assert_eq!(Some(0), response.content_length())
            }
            Err(why) => { println!("Failed to send request {why}") }
        }
    }

    fn spawn_app() {
        match run() {
            Ok(server) => {
                tokio::spawn(server);
            }
            Err(why) => { println!("Failed to spawn server: {}", why) }
        }
    }
}
