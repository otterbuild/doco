//! 🦕 doco
//!
//! `doco` is a framework and runner for end-to-tests of web applications.

pub use anyhow::{anyhow, Context, Error, Result};
pub use doco_derive::{main, test};
pub use fantoccini::Locator;
use getset::Getters;
pub use inventory;
pub use testcontainers::core::WaitFor;
use typed_builder::TypedBuilder;

pub use crate::client::Client;
pub use crate::server::Server;
pub use crate::service::Service;
pub use crate::test_runner::TestRunner;

mod client;
mod environment;
mod server;
mod service;
mod test_runner;

#[cfg(test)]
mod test_utils;

#[derive(Clone, Debug, Default, Getters, TypedBuilder)]
pub struct Doco {
    #[getset(get = "pub")]
    server: Server,

    #[builder(via_mutators(init = Vec::new()), mutators(
        pub fn service(mut self, service: Service) {
            self.services.push(service);
        }
    ))]
    #[getset(get = "pub")]
    services: Vec<Service>,
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::Doco;

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
