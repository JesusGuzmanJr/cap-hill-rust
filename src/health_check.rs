#![cfg(feature = "ssr")]

use {actix_web::rt::task::JoinHandle, anyhow::Result, std::time::Duration};

const INTERVAL: Duration = Duration::from_secs(15);

pub fn spawn_success_loop(health_check_ping_url: String) {
    actix_web::rt::spawn(async move {
        let client = reqwest::Client::new();
        loop {
            let response = client.post(&health_check_ping_url).send().await?;
            let status = response.status();
            let body = response.text().await?;
            if body != "OK" {
                tracing::error!(?status, body, "health check failed");
                anyhow::bail!("health check failed: {status}: {body}");
            }
            actix_web::rt::time::sleep(INTERVAL).await;
        }
        Ok::<_, anyhow::Error>(())
    });
}
