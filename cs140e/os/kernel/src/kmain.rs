#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(decl_macro)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(rustc_private)]

use pi::{gpio::Gpio, uart::MiniUart};
use core::fmt::Write;

extern crate pi;
extern crate stack_vec;
extern crate core;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

#[no_mangle]
pub extern "C" fn kmain() {
    let mut pin16 = Gpio::new(16).into_output();
    let mut uart = MiniUart::new();
    
    // infinite loop
    loop {
        // set pin 16 (turn it on)
        pin16.set();
        uart.write_str("LED is on!\n");
        pi::timer::spin_sleep_ms(500);

        // clear pin 16 (turn it off)
        pin16.clear();
        uart.write_str("LED is off!\n");
        pi::timer::spin_sleep_ms(500);
    }
}
