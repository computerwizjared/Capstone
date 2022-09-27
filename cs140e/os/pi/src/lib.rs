#![feature(core_intrinsics)]
#![feature(rustc_private)]
#![feature(decl_macro)]
#![feature(never_type)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;
extern crate volatile;

pub mod timer;
pub mod uart;
pub mod gpio;
pub mod common;
