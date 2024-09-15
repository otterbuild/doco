//! 🦕 doco
//!
//! `doco` is a framework and runner for end-to-tests of web applications.

pub use anyhow::{Context, Error, Result};
pub use doco_types::*;
pub use fantoccini::Locator;

pub use crate::client::Client;
pub use crate::test_runner::*;

mod client;
mod test_runner;

#[cfg(test)]
mod test_utils;
