#[allow(unused)]
use crate::tests::integration_test::integration_tests;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = integration_tests::spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    match client
        .post(format!("{}/subscriptions", address))
        .body(body)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await
    {
        Ok(response) => {
            assert_eq!(200, response.status().as_u16())
        }
        Err(why) => {
            println!("failed to execute request: {why}")
        }
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = integration_tests::spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, error_message) in test_cases {
        match client
            .post(format!("{}/subscriptions", address))
            .body(body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await
        {
            Ok(response) => {
                assert_eq!(
                    400,
                    response.status().as_u16(),
                    "The API did not fail with 400 Bad Request when the payload was {}",
                    error_message
                );
            }
            Err(why) => {
                println!("failed to execute request: {why}")
            }
        }
    }
}
