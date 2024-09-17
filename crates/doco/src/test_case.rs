use getset::CopyGetters;

use crate::{Client, Result};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, CopyGetters)]
pub struct TestCase {
    #[getset(get_copy = "pub")]
    function: fn(Client) -> Result<()>,
}

impl TestCase {
    pub fn new(function: fn(Client) -> Result<()>) -> Self {
        Self { function }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_send() {
        assert_send::<TestCase>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<TestCase>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<TestCase>();
    }
}
