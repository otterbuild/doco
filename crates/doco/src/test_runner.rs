use std::time::Duration;

use anyhow::Context;
use testcontainers::core::{Host, IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use tokio::time::sleep;

use crate::{Client, Doco, Result};

const DOCKER_HOST: &str = "host.docker.internal";

#[derive(Debug)]
pub struct TestRunner {
    doco: Doco,
    selenium: ContainerAsync<GenericImage>,
}

impl TestRunner {
    pub async fn init(doco: Doco) -> Result<Self> {
        println!("Initializing ephemeral test environment...");

        let selenium = start_selenium().await?;

        Ok(Self { doco, selenium })
    }

    pub async fn run(&self, name: &str, test: fn(Client) -> Result<()>) -> Result<()> {
        let mut services = Vec::with_capacity(self.doco.services().len());

        let mut server = GenericImage::new(self.doco.server().image(), self.doco.server().tag())
            .with_exposed_port(self.doco.server().port().tcp())
            .with_wait_for(WaitFor::seconds(5))
            .with_host(DOCKER_HOST, Host::HostGateway);

        for service in self.doco.services() {
            let mut image = GenericImage::new(service.image(), service.tag());

            if let Some(wait) = service.wait() {
                image = image.with_wait_for(wait.clone());
            }

            let mut image = image.with_host("doco", Host::HostGateway);

            for env in service.envs() {
                image = image.with_env_var(env.name().clone(), env.value().clone());
            }

            let container = image.start().await?;

            server = server.with_host(
                service.image(),
                Host::Addr(container.get_bridge_ip_address().await?),
            );

            services.push(container);
        }

        let server = server.start().await?;
        let stdout = String::from_utf8(server.stderr_to_vec().await?)?;
        println!("{stdout}");

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
            .base_url(format!("http://{DOCKER_HOST}:{port}").parse()?)
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

async fn start_selenium() -> Result<ContainerAsync<GenericImage>> {
    GenericImage::new("selenium/standalone-firefox", "latest")
        .with_exposed_port(4444.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Started Selenium Standalone"))
        .with_host(DOCKER_HOST, Host::HostGateway)
        .start()
        .await
        .context("failed to start Selenium container")
}

#[cfg(test)]
mod tests {
    use axum::routing::get;
    use axum::Router;
    use tokio::net::TcpListener;

    use crate::test_utils::*;
    use crate::Result;

    use super::*;

    #[tokio::test]
    async fn selenium_can_access_host() -> Result<()> {
        let listener = TcpListener::bind("0.0.0.0:0").await?;
        let port = listener.local_addr()?.port();

        let app = Router::new().route("/", get(|| async { "hello from the test" }));
        tokio::spawn(async { axum::serve(listener, app).await });

        let selenium = start_selenium().await?;

        let client = fantoccini::ClientBuilder::native()
            .connect(&format!(
                "http://{}:{}",
                selenium.get_host().await?,
                selenium.get_host_port_ipv4(4444).await?
            ))
            .await
            .expect("failed to connect to WebDriver");

        client
            .goto(&format!("http://{DOCKER_HOST}:{port}/"))
            .await?;
        let body = client.source().await?;

        assert!(body.contains("hello from the test"));

        Ok(())
    }

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
