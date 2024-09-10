use fantoccini::error::CmdError;
use fantoccini::Client as WebDriverClient;
use reqwest::Url;
use std::ops::Deref;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, TypedBuilder)]
pub struct Client {
    base_url: Url,
    client: WebDriverClient,
}

impl Client {
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
