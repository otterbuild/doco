//! 🦕 doco
//!
//! `doco` is a framework and runner for end-to-tests of web applications.

use std::future::Future;
use std::time::Duration;

pub use anyhow::{Context, Error, Result};
pub use fantoccini::{Client, Locator};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::GenericImage;
use tokio::time::sleep;
use typed_builder::TypedBuilder;

use crate::server::Server;

pub mod server;

#[cfg(test)]
mod test_utils;

pub trait TestCase {
    fn execute(&self, client: Client, host: String, port: u16) -> impl Future<Output = Result<()>>;
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
pub struct Doco {
    server: Server,
}

impl Doco {
    pub async fn run<F>(&self, test: F) -> Result<()>
    where
        F: TestCase,
    {
        let selenium = GenericImage::new("selenium/standalone-firefox", "latest")
            .with_exposed_port(4444.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Started Selenium Standalone"))
            .start()
            .await?;

        let container = GenericImage::new("doco", "leptos")
            .with_exposed_port(self.server.port().tcp())
            .start()
            .await?;

        let host = container.get_host().await?;
        let port = container.get_host_port_ipv4(self.server.port()).await?;

        let client = fantoccini::ClientBuilder::native()
            .connect(&format!(
                "http://{}:{}",
                selenium.get_host().await?,
                selenium.get_host_port_ipv4(4444).await?
            ))
            .await
            .expect("failed to connect to WebDriver");

        for _ in 0..10 {
            if reqwest::Client::new()
                .get(format!("http://{host}:{port}/"))
                .send()
                .await
                .is_ok()
            {
                break;
            } else {
                sleep(Duration::from_secs(1)).await;
            }
        }

        test.execute(
            client,
            container.get_bridge_ip_address().await?.to_string(),
            self.server.port(),
        )
        .await?;

        container.stop().await?;
        selenium.stop().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_send() {
        assert_send::<Doco>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Doco>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Doco>();
    }
}
