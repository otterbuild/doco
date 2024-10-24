//! WebDriver client that interacts with the web application

use fantoccini::error::CmdError;
use fantoccini::Client as WebDriverClient;
use reqwest::Url;
use std::ops::Deref;
use typed_builder::TypedBuilder;

/// WebDriver client that interacts with the web application
///
/// The `Client` implements the [WebDriver] protocol to interact with the web application. It is
/// preconfigured to target the server that has been passed to Doco so that users only need to
/// supply the path that they want to test.
///
/// Internally, the client uses the [fantoccini] crate to interact with the WebDriver server. For
/// examples on what methods are available on the `Client` and how to interact with the web
/// application, see the [fantoccini] documentation.
///
/// # Example
///
/// ```rust
/// use doco::{Client, Result};
///
/// #[doco::test]
/// async fn visit_root_path(client: Client) -> Result<()> {
///     client.goto("/").await?;
///
///     let body = client.source().await?;
///
///     assert!(body.contains("Hello World"));
///
///     Ok(())
/// }
/// #
/// # use doco::{Doco, Server};
/// #
/// # #[doco::main]
/// # async fn main() -> Doco {
/// #    let server = Server::builder()
/// #        .image("crccheck/hello-world")
/// #        .tag("v1.0.0")
/// #        .port(8000)
/// #        .build();
/// #
/// #    Doco::builder().server(server).build()
/// # }
/// ```
///
/// [fantoccini]: https://crates.io/crates/fantoccini
/// [webdriver]: https://developer.mozilla.org/en-US/docs/Web/WebDriver
#[derive(Clone, Debug, TypedBuilder)]
pub struct Client {
    /// The base URL of the server
    base_url: Url,

    /// The WebDriver client that is used internally
    client: WebDriverClient,
}

impl Client {
    /// Navigate to the specified path
    ///
    /// This method will navigate to the specified path on the server. The path should be relative
    /// to the base URL that was passed to Doco.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use doco::{Client, Result};
    ///
    /// async fn visit_root_path(client: Client) -> Result<()> {
    ///     client.goto("/").await?;
    ///
    ///     // Interact with the web application and make assertions
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn goto(&self, path: &str) -> Result<(), CmdError> {
        self.client.goto(self.base_url.join(path)?.as_str()).await
    }
}

impl Deref for Client {
    type Target = WebDriverClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_send() {
        assert_send::<Client>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Client>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Client>();
    }
}
