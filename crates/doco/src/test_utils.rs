//! Utilities for testing this crate

/// Asserts that a type implements `Send`
pub fn assert_send<T: Send>() {}

/// Asserts that a type implements `Sync`
pub fn assert_sync<T: Sync>() {}

/// Asserts that a type implements `Unpin`
pub fn assert_unpin<T: Unpin>() {}
