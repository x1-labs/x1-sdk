#![deprecated(since = "2.3.0", note = "Use `solana_sdk_ids::system_program` instead")]
//! The [system native program][np].
//!
//! [np]: https://docs.solanalabs.com/runtime/programs#system-program
pub use solana_sdk_ids::system_program::{check_id, id, ID};
