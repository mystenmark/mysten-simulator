//! Client implementation and builder.

use super::Error;
use std::time::Duration;

/// Channel builder.
pub struct Endpoint {}

impl Endpoint {
    /// Apply a timeout to connecting to the uri.
    ///
    /// Defaults to no timeout.
    pub fn connect_timeout(self, dur: Duration) -> Self {
        todo!()
    }

    /// Create a channel from this config.
    pub async fn connect(&self) -> Result<Channel, Error> {
        todo!()
    }
}

impl TryFrom<&'static str> for Endpoint {
    type Error = Error;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<String> for Endpoint {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// A default batteries included `transport` channel.
#[derive(Clone)]
pub struct Channel {}