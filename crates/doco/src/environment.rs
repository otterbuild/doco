use getset::Getters;
use typed_builder::TypedBuilder;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Getters, TypedBuilder)]
pub struct Variable {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    name: String,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    value: String,
}

impl Variable {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Variable {
        Variable {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_send() {
        assert_send::<Variable>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Variable>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Variable>();
    }
}
