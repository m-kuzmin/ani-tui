/// Provides utility functionality used throughout the crate
pub mod core;

/// Implementation of app's features
pub mod features;

#[macro_use(async_trait)]
extern crate async_trait;

#[macro_use(Subcommand, Args)]
extern crate clap;

#[cfg(test)]
#[macro_use(automock, mock)]
extern crate mockall;

#[cfg(test)]
#[macro_use(double)]
extern crate mockall_double;
