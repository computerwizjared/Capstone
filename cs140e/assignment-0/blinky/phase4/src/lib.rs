#![feature(compiler_builtins, lang_items, rustc_private)]
#![no_builtins]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { core::arch::asm!("nop"); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // 18 is the lower bit for pin 16 (bits 20-18)
    // | is bitwise OR operator
    GPIO_FSEL1.write_volatile(GPIO_FSEL1.read_volatile() | (0b001 << 18));
    
    // infinite loop
    loop {
        // set pin 16 (turn it on)
        GPIO_SET0.write_volatile(GPIO_SET0.read_volatile() | (0b1 << 16));
        spin_sleep_ms(500);

        // clear pin 16 (turn it off)
        GPIO_CLR0.write_volatile(GPIO_CLR0.read_volatile() | (0b1 << 16));
        spin_sleep_ms(500);
    }
}
