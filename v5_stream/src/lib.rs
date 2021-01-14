#![no_std]

#[cfg(feature = "std")]
extern crate std;
extern crate alloc;

pub mod multiplexed_stream;

pub mod composed_stream;

//TODO: Make a binary packet handler
