#![cfg_attr(not(std), no_std)]
#![feature(asm)]
#![feature(never_type)]
#![feature(const_fn)]
#![feature(get_mut_unchecked)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate log;

pub mod allocator;
pub mod device;
pub mod error;
pub mod memory;
mod percpu;
mod registers;
pub mod vm;
pub mod vmcs;
mod vmexit;
pub mod vmx;
