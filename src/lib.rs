mod core;
mod features;
#[macro_use(async_trait)]
extern crate async_trait;

#[macro_use(lazy_static)]
extern crate lazy_static;

#[macro_use(Error)]
extern crate thiserror;

#[cfg(test)]
#[macro_use(automock)]
extern crate mockall;
