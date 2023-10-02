#[macro_use]
extern crate cli_log;

mod cli;
mod errors;
mod wallker;
mod dir_stat;
mod file_reader;

// #[global_allocator]
// static ALLOC: leak::LeakingAllocator = leak::LeakingAllocator::new();

pub use {
    cli::*,
    errors::*,
    wallker::*,
    dir_stat::*,
    file_reader::*,
};
