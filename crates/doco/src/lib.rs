//! 🦕 doco
//!
//! `doco` is a framework and runner for end-to-tests of web applications.

use std::future::Future;

pub use anyhow::{Context, Error, Result};
use testcontainers::core::IntoContainerPort;
use testcontainers::runners::AsyncRunner;
use testcontainers::GenericImage;
use typed_builder::TypedBuilder;

use crate::server::Server;

pub mod server;

#[cfg(test)]
mod test_utils;

pub trait TestCase {
    fn execute(&self, host: String, port: u16) -> impl Future<Output = Result<()>>;
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
        let container = GenericImage::new("doco", "leptos")
            .with_exposed_port(self.server.port().tcp())
            .start()
            .await?;

        let host = container.get_host().await?;
        let port = container.get_host_port_ipv4(self.server.port()).await?;

        test.execute(host.to_string(), port).await?;

        container.stop().await?;

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
