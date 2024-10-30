use tokio;

#[cfg(test)]
mod e2e_test {
    use super::*;

    #[tokio::test]
    async fn test() {
        let client = reqwest::Client::new();
        let response = client.get("http://server:8000")
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success());
    }

}