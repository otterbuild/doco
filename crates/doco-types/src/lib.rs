use getset::Getters;
use typed_builder::TypedBuilder;

pub use self::server::*;

mod server;

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
