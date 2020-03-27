#![allow(deprecated)]
pub mod args;
pub mod read;
pub mod stats;
pub mod write;

const CHUNK_SIZE: usize = 7 * 1024;
