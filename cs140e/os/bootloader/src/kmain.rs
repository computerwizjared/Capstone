#![feature(lang_items)]
#![feature(rustc_private)]

use core::arch::asm;
use core::slice;

use pi::uart;

extern crate core;
extern crate pi;
extern crate xmodem;

pub mod lang_items;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        asm!("br {}", in(reg) addr as usize);
        loop {
            asm!("nop")
        }
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    // FIXME: Implement the bootloader.
    let mut uart = uart::MiniUart::new();
    uart.set_read_timeout(750);
    let mut available_memory: &mut [u8] =
        unsafe { slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE) };

    loop {
        let result = xmodem::Xmodem::receive(&mut uart, &mut available_memory);
        match result {
            Ok(_) => {
                // move on to jump at end
                break;
            }
            Err(_) => {
                continue;
            }
        }
    }

    jump_to(BINARY_START);
}
