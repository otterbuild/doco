use getset::{CopyGetters, Getters};
use testcontainers::core::WaitFor;
use typed_builder::TypedBuilder;

use crate::environment::Variable;

#[derive(Clone, Debug, Default, CopyGetters, Getters, TypedBuilder)]
pub struct Service {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    image: String,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    tag: String,

    #[builder(via_mutators(init = Vec::new()), mutators(
        pub fn env(mut self, name: impl Into<String>, value: impl Into<String>) {
            self.envs.push(Variable::new(name, value));
        }
    ))]
    #[getset(get = "pub")]
    envs: Vec<Variable>,

    #[builder(default, setter(into))]
    #[getset(get = "pub")]
    wait: Option<WaitFor>,
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn env_collects_variables() {
        let service = Service::builder()
            .image("postgres")
            .tag("latest")
            .env("POSTGRES_PASSWORD", "password")
            .env("POSTGRES_USER", "postgres")
            .env("POSTGRES_PASSWORD", "postgres")
            .build();

        assert_eq!(3, service.envs.len());
    }

    #[test]
    fn trait_send() {
        assert_send::<Service>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Service>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Service>();
    }
}
