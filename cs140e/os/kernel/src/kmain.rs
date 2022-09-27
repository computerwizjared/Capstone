#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(decl_macro)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(rustc_private)]

extern crate pi;
extern crate stack_vec;
extern crate core;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

#[no_mangle]
pub extern "C" fn kmain() {
    // FIXME: Start the shell.
}
