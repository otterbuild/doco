use std::time::Duration;

use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage};
use tokio::time::sleep;

use crate::{Client, Doco, Result};

#[derive(Debug)]
pub struct TestRunner {
    doco: Doco,
    selenium: ContainerAsync<GenericImage>,
}

impl TestRunner {
    pub async fn init(doco: Doco) -> Result<Self> {
        println!("Initializing ephemeral test environment...");

        let selenium = GenericImage::new("selenium/standalone-firefox", "latest")
            .with_exposed_port(4444.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Started Selenium Standalone"))
            .start()
            .await?;

        Ok(Self { doco, selenium })
    }

    pub async fn run(&self, name: &str, test: fn(Client) -> Result<()>) -> Result<()> {
        let server = GenericImage::new("doco", "leptos")
            .with_exposed_port(self.doco.server().port().tcp())
            .start()
            .await?;

        let host = server.get_host().await?;
        let port = server.get_host_port_ipv4(self.doco.server().port()).await?;
        let server_endpoint = format!("http://{host}:{port}/");

        let client = fantoccini::ClientBuilder::native()
            .connect(&format!(
                "http://{}:{}",
                self.selenium.get_host().await?,
                self.selenium.get_host_port_ipv4(4444).await?
            ))
            .await
            .expect("failed to connect to WebDriver");

        let client = Client::builder()
            .base_url(
                format!(
                    "http://{}:{}",
                    server.get_bridge_ip_address().await?,
                    self.doco.server().port(),
                )
                .parse()?,
            )
            .client(client)
            .build();

        for _ in 0..10 {
            if reqwest::Client::new()
                .get(&server_endpoint)
                .send()
                .await
                .is_ok()
            {
                break;
            } else {
                sleep(Duration::from_secs(1)).await;
            }
        }

        println!("{}...", name);
        test(client)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_send() {
        assert_send::<TestRunner>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<TestRunner>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<TestRunner>();
    }
}
