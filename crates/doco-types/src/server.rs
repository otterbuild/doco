use getset::{CopyGetters, Getters};
use typed_builder::TypedBuilder;

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, CopyGetters, Getters, TypedBuilder,
)]
pub struct Server {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    image: String,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    tag: String,

    #[getset(get_copy = "pub")]
    port: u16,
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
