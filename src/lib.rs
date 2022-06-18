#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]
#![allow(unused_macros)]

/// Command line interface
pub mod cli_args;

/// Interface to anime data sources such as searching and getting a watch link
pub mod anime_repo;

/// Contains Website specific APIs and their repo impls
pub mod websites {
    /// <https://goload.pro> API
    pub mod gogoplay;
}

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
