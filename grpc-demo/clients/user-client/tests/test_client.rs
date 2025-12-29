#[cfg(test)]
mod client_test {

    #[tokio::test]
    async fn client_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let base_url = "http://127.0.0.1:8081";

        println!("🚀 Testing User Client Service HTTP API");

        let health_resp = client.get(&format!("{}/health", base_url)).send().await?;
        println!(
            "Health check: {} - {:?}",
            health_resp.status(),
            health_resp.text().await?
        );
        Ok(())
    }
}
