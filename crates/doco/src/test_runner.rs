use std::future::Future;
use std::time::Duration;

use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::GenericImage;
use tokio::time::sleep;
use typed_builder::TypedBuilder;

use crate::{Client, Doco};

pub trait TestCase {
    fn execute(&self, client: Client) -> impl Future<Output = anyhow::Result<()>>;
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
pub struct TestRunner {
    doco: Doco,
}

impl TestRunner {
    pub async fn run<F>(&self, test: F) -> anyhow::Result<()>
    where
        F: TestCase,
    {
        let selenium = GenericImage::new("selenium/standalone-firefox", "latest")
            .with_exposed_port(4444.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Started Selenium Standalone"))
            .start()
            .await?;

        let container = GenericImage::new("doco", "leptos")
            .with_exposed_port(self.doco.server().port().tcp())
            .start()
            .await?;

        let host = container.get_host().await?;
        let port = container
            .get_host_port_ipv4(self.doco.server().port())
            .await?;

        let client = fantoccini::ClientBuilder::native()
            .connect(&format!(
                "http://{}:{}",
                selenium.get_host().await?,
                selenium.get_host_port_ipv4(4444).await?
            ))
            .await
            .expect("failed to connect to WebDriver");

        let client = Client::builder()
            .base_url(
                format!(
                    "http://{}:{}",
                    container.get_bridge_ip_address().await?,
                    self.doco.server().port(),
                )
                .parse()?,
            )
            .client(client)
            .build();

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

        test.execute(client).await?;

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
