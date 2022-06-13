#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]
#![allow(unused_macros)]

/// Provides utility functionality used throughout the crate
pub mod core;

/// Implementation of app's features
pub mod features;

#[macro_use(async_trait)]
extern crate async_trait;

#[macro_use(Subcommand, Args)]
extern crate clap;

#[macro_use(with)]
extern crate with_macro;

#[cfg(test)]
#[macro_use(automock, mock)]
extern crate mockall;

#[cfg(test)]
#[macro_use(double)]
extern crate mockall_double;
