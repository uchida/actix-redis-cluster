//! Redis integration for Actix framework.
//!
//! ## Documentation
//! * [API Documentation (Development)](http://actix.github.io/actix-redis/actix_redis/)
//! * [API Documentation (Releases)](https://docs.rs/actix-redis/)
//! * [Chat on gitter](https://gitter.im/actix/actix)
//! * Cargo package: [actix-redis](https://crates.io/crates/actix-redis)
//! * Minimum supported Rust version: 1.26 or later
//!
extern crate actix;
extern crate backoff;
extern crate futures;
extern crate tokio_codec;
extern crate tokio_io;
extern crate tokio_tcp;
#[macro_use]
extern crate log;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate failure;
extern crate time;

mod cluster;
pub mod command;
mod redis;
mod slot;
pub use cluster::RedisClusterActor;
pub use redis::RedisActor;

#[cfg(feature = "web")]
extern crate actix_web;
#[cfg(feature = "web")]
extern crate cookie;
#[cfg(feature = "web")]
extern crate http;
#[cfg(feature = "web")]
extern crate rand;
#[cfg(feature = "web")]
extern crate serde;
#[cfg(feature = "web")]
extern crate serde_json;

#[cfg(feature = "web")]
mod session;
#[cfg(feature = "web")]
pub use cookie::SameSite;
#[cfg(feature = "web")]
pub use session::RedisSessionBackend;

/// General purpose actix redis error
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Redis error {}", _0)]
    Redis(redis_async::error::Error),
    /// Receiving message during reconnecting
    #[fail(display = "Redis: Not connected")]
    NotConnected,
    /// Cancel all waters when connection get dropped
    #[fail(display = "Redis: Disconnected")]
    Disconnected,
    /// Trying to access multiple slots at once in cluster mode
    #[fail(display = "Redis: Multiple slot command {:?}", _0)]
    MultipleSlot(Vec<u16>),
}

impl From<redis_async::error::Error> for Error {
    fn from(err: redis_async::error::Error) -> Error {
        Error::Redis(err)
    }
}

// re-export
pub use redis_async::error::Error as RespError;
pub use redis_async::resp::RespValue;
