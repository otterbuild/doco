//! Server for the web application that is being tested

use getset::{CopyGetters, Getters};
use testcontainers::core::WaitFor;
use typed_builder::TypedBuilder;

/// Server for the web application that is being tested
///
/// The `Server` struct configures the server that is being tested. Doco runs the server as a Docker
/// container, using a prebuilt image.
#[derive(Clone, Debug, CopyGetters, Getters, TypedBuilder)]
pub struct Server {
    /// The name of the Docker image for the server, e.g. `rust`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    image: String,

    /// The tag for the Docker image, e.g. `latest`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    tag: String,

    /// The port that the server listens on, e.g. `8080`
    #[getset(get_copy = "pub")]
    port: u16,

    /// An optional condition to wait until the server has properly started
    #[builder(default, setter(into))]
    #[getset(get = "pub")]
    wait: Option<WaitFor>,
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_send() {
        assert_send::<Server>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Server>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Server>();
    }
}
