//! This crate allows for simple, fast and reliable communication with the FCM to send app notifications.
//! It contains logic to rotate the tokens in a fast and memory efficient way.
//!
//! ## FCM Context Object
//! The main handler of your FCM project is the context object found in the `fcm` module.
//! It's used to hold the token manager and send notifications.
//! Each of its functions doesn't require mutable access, so it can easily be used globally by lazy loading.
//!
//! There are no Mutexes or unsafe code, everything is based on atomics and message passing channels.
//! Only the token itself is behind an RwLock.
//!
//! ## FCM Requests
//! To build a notification use the structures in `request` module.
//! There are necessities to build a custom notification request with user defined data.

/// Module for building FCM notifications.
pub mod request;

/// FCM context object module.
pub mod fcm;

/// Errors that can occur when using the `fcm-n` crate.
mod error;

pub use error::*;
