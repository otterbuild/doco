pub fn assert_send<T: Send>() {}

pub fn assert_sync<T: Sync>() {}

pub fn assert_unpin<T: Unpin>() {}
