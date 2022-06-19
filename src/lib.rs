/*! # AniTUI - An app for watching anime in MPV
* See readme on github or crates.io for usage documentation.
*/

#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![allow(unused_macros)]

/// Abstracts the CLI API
pub mod cli_args;

/// Contains anime data source abstactions
pub mod anime_repo;

/// Contains implementations of [`anime_repo`]
pub mod websites {
    /// <https://goload.pro> API
    pub mod gogoplay;
}

#[macro_use(async_trait)]
extern crate async_trait;

#[macro_use(Subcommand)]
extern crate clap;

#[macro_use(with)]
extern crate with_macro;

#[cfg(test)]
#[macro_use(automock, mock)]
extern crate mockall;

#[cfg(test)]
#[macro_use(double)]
extern crate mockall_double;
