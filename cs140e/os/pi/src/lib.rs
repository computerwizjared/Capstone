#![feature(core_intrinsics)]
#![feature(rustc_private)]
#![feature(decl_macro)]
#![feature(never_type)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;
extern crate display_interface_i2c;
extern crate stack_vec;
extern crate volatile;

pub mod common;
pub mod gpio;
pub mod i2c;
pub mod timer;
pub mod uart;
