#[allow(unused)]
mod tests {
    use crate::health_checker::health_check_handler;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check_handler().await;

        assert!(response.status().is_success())
    }
}
