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

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // 18 is the lower bit for pin 16 (bits 20-18)
    // | is bitwise OR operator
    GPIO_FSEL1.write_volatile(GPIO_FSEL1.read_volatile() | (0b001 << 18));
    
    // infinite loop
    loop {
        // set pin 16 (turn it on)
        GPIO_SET0.write_volatile(GPIO_SET0.read_volatile() | (0b1 << 16));
        pi::timer::spin_sleep_ms(500);

        // clear pin 16 (turn it off)
        GPIO_CLR0.write_volatile(GPIO_CLR0.read_volatile() | (0b1 << 16));
        pi::timer::spin_sleep_ms(500);
    }
}
