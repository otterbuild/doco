//! 🦕 doco
//!
//! `doco` is a framework and runner for end-to-tests of web applications.

pub use anyhow::{Context, Error, Result};
pub use fantoccini::Locator;
use getset::Getters;
use typed_builder::TypedBuilder;

pub use crate::client::Client;
pub use crate::server::Server;
pub use crate::test_case::TestCase;
pub use crate::test_runner::TestRunner;

mod client;
mod server;
mod test_case;
mod test_runner;

#[cfg(test)]
mod test_utils;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Getters, TypedBuilder)]
pub struct Doco {
    #[getset(get = "pub")]
    server: Server,
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
