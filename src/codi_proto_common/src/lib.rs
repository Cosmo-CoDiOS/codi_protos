//! This crate handles UART communication between the `CoDi` chip and the host ROM.
#![cfg_attr(any(target_arch = "arm"), no_std)]
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
//    unused_extern_crates, temporarily disable!
    variant_size_differences
)]

#[cfg(target_arch = "arm")]
extern crate alloc;
#[cfg(target_arch = "arm")]
extern crate core;

#[cfg(not(feature = "runs-on-stm32")]
pub mod std_serial_handler;
