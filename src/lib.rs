pub mod core;
pub mod features;

#[macro_use(async_trait)]
extern crate async_trait;

#[macro_use(lazy_static)]
extern crate lazy_static;

#[macro_use(Error)]
extern crate thiserror;

#[macro_use(Parser, Subcommand, Args)]
extern crate clap;

#[cfg(test)]
#[macro_use(automock, mock)]
extern crate mockall;

#[cfg(test)]
#[macro_use(double)]
extern crate mockall_double;
