# Assignment 0

I began this project by following the steps located at the CS140e course [here](https://cs140e.sergio.bz/assignments/0-blinky/). All steps in this documentation are executed within the `cs140e/assignment-0` directory. I am using an Adafruit USB to TTL Serial Cable from [here](https://www.adafruit.com/product/954) which is compatible with the same driver used in the CS140e course. I also use a generic USB-C to USB-A adapter, since my MacBook only has USB-C ports and the cable is USB-A.

## Phase 0

I cloned the "blinky" skeleton project from the CS140e Git repository into this repository under the `blinky` folder.

```
git clone https://cs140e.sergio.bz/assignments/0-blinky/skeleton.git blinky
rm -rf blinky/.git
```

Then I ran the commands instructed to setup the assignment:

```
cd blinky
make fetch
```

However, I ran into this issue:

```
wget https://cs140e.sergio.bz/assignments/0-blinky/data/firmware.tar.gz -O files/firmware.tar.gz
make: wget: No such file or directory
make: *** [files/firmware.tar.gz] Error 1
```

I found out that modern versions of macOS don't come pre-installed with `wget`.
Rather than install `wget`, I adjusted the `Makefile` to use `curl`.

```diff
$(ASSIGNMENT_FILES): | $(FILES_DIR)
-	wget $(BASE_URL)/assignments/0-blinky/data/$(@:$(FILES_DIR)/%=%) -O $@
+	curl $(BASE_URL)/assignments/0-blinky/data/$(@:$(FILES_DIR)/%=%) -o $@
```

To include the `files` directory in this Git repository (in case the original source goes offline),
I modified the `.gitignore` file to include it:

```diff
# Assignment 0 specific
phase4/build
phase4/target
-files/
+-files/**
```

## Phase 1

Since I run macOS, I began by installing the `Silicon Labs VPC Driver` for macOS from [here](https://www.silabs.com/documents/public/software/Mac_OSX_VCP_Driver.zip).

I then connected my Pi to my USB to Serial cable using this GPIO diagram and table:

![Diagram of Raspberry Pi GPIO pinout](assets/pi3-gpio-pinout.svg?raw=true&sanitize=true)

| GPIO Pin (Function) | USB to Serial Wire Color (Function) |
| --- | --- |
| 4 (5V PWR) | Red (5V @ 500mA) |
| 6 (GND) | Black (Ground) |
| 8 (UART_TXD0) | White (RX) |
| 10 (UART_RXD0) | Green (TX) |

I then plugged in my USB to Serial cable to my MacBook, and saw the red power LED on the Pi light up, which means that the Pi powered on properly! I ran this command on my MacBook to determine the serial device:

```
ls /dev/tty.*
```

I found that my USB to Serial cable device is located at `/dev/tty.usbserial-0001`.

Here is a picture of my setup!

![Picture of the Raspberry Pi plugged into MacBook](assets/serial-setup.jpg?raw=true)

Next, unplugged my Pi, inserted my SD card into my MacBook, and opened Disk Utility.
I selected the SD card, and clicked `Erase` in the top bar of the window.
I named the SD card `BOOT` and selected `MS-DOS (FAT)` as the format, as shown below, and clicked `Erase`.

![Screenshot of Disk Utility](assets/erase-sd-card.png?raw=true)

I then copied `blinky/files/firmware/*` to the root of the SD card.
I also copied `blinky/files/activity-led-blink.bin` to the root of the SD card, and renamed it to `kernel8.img`.
I ejected my SD card, and inserted it into my Pi.

Then, I plugged my Pi back into my MacBook.
I saw the red light come back on, but also saw a flashing green light!
This means that the custom OS image worked!

Then, I ran this command to connect to the Pi over the serial console:

```
screen /dev/tty.usbserial-0001 115200
```

Note, `/dev/tty.usbserial-0001` is the device of the USB to Serial cable from earlier, and 115200 is the baud rate of the connection.

I then saw the console output below indicating the LED status, which means the serial connection worked!

```
On!
Off!
On!
Off!
```

I then used `Ctrl+A`, then `K`, then `y` to exit the serial console, and unplugged my Pi from my MacBook.

## Phase 2

With my Raspberry Pi disconnected from my MacBook, I set up the breadboard according to this diagram.
I only had a 1.6k ohm resistor, but it worked fine in this case.

![Diagram of Raspberry Pi and Breadboard for Always On LED](assets/always-on-led-circuit.svg?raw=true&sanitize=true)

I plugged in the Pi to my MacBook, and the LED lit up, as it was supposed to!

![Photo of Raspberry Pi and Breadboard for Always On LED](assets/always-on-led-photo.jpg?raw=true)

I then unplugged the Pi, and set up the breadboard according to this diagram.

![Diagram of Raspberry Pi and Breadboard for Blinking LED](assets/gpio-16-led-circuit.svg?raw=true&sanitize=true)

I put the SD card into my MacBook, and copied `blinky/files/gpio16-blink.bin` to it, renaming it to `kernel8.img`.
I put the SD card back into the Pi, plugged it back in, and the LED started blinking rapidly, which means all is working!
I also discovered that if I connect to the Pi using the serial console again, it displays this output:

```
Blink on...
Blink off...
Blink on...
Blink off...
```

## Phase 3

First, I installed Xcode tools on my MacBook using this command:

```
xcode-select --install
```

I also installed Homebrew using this command:

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

I then installed the 64-bit ARM toolchain to be able to compile an OS for the Raspberry Pi.
I decided to use `gcc-aarch64-embedded` rather than the `SergioBenitez/osxct` (CS140e-provided) package because it is the official ARM package and is more up-to-date.

```
$ brew install --cask gcc-aarch64-embedded
```

I verified it was installed correctly:

```
aarch64-none-elf-gcc --version
```

```
aarch64-none-elf-gcc (GNU Toolchain for the Arm Architecture 11.2-2022.02 (arm-11.14)) 11.2.1 20220111
Copyright (C) 2021 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
```

I have included the CS140e-provided documentation here for reference, in case the original site goes down.
This is similar to the BCM2835 documentation, with some edits specific to the BCM2837 (the Pi 3B processor).

According to the documentation, the BCM2837 has a base peripheral address of `0x7E000000` which is at physical address 0x3F000000.
This means to control the GPIO pins, we need to use this address.
Here is documentation specific to the registers used in this assignment:

| name | peripheral address | description | size | read/write |
| --- | --- | --- | --- | --- |
| GPFSEL1 | 0x7E200004 | GPIO Function Select 1 | 32 bits | R/W |
| GPSET0 | 0x7E20001C | GPIO Pin Output Set 0 | 32 bits | W |
| GPCLR0 | 0x7E200028 | GPIO Pin Output Clear 0 | 32 bits | W |

To set pin 16 as an output, we need to write `001` to field FSEL16 (bits 20-18) in register GPFSEL1 (address `0x7E200004`).
To set pin 16 (turn it on), we need write `1` to field SET16 (bit 16) in register GPSET0 (address `0x7E20001C`).
To clear pin 16 (turn it off), we need to write `1` to field CLR16 (bit 16) in register GPCLR0 (address `0x7E200028`).

I then opened the `blinky/phase3/blinky.c` file, and edited it to add the blink functionality.

```diff
int main(void) {
-  // FIXME: STEP 1: Set GPIO Pin 16 as output.
+  // 18 is the lower bit for pin 16 (bits 20-18)
+  // | is bitwise OR operator
+  *GPIO_FSEL1 |= (0b001 << 18); 
+  
-  // FIXME: STEP 2: Continuously set and clear GPIO 16.
+  // infinite loop
+  for (;;) {
+    // set pin 16 (turn it on)
+    *GPIO_SET0 |= (0b1 << 16);
+    spin_sleep_ms(500);
+    
+    // clear pin 16 (turn it off)
+    *GPIO_CLR0 |= (0b1 << 16);
+    spin_sleep_ms(500);
+  }
}
```

Then I ran `make` in the `blinky/phase3` directory, and copied `blinky.bin` to `kernel8.img` on the SD card.
I plugged in the Pi to my MacBook and the LED started blinking!

## Phase 4

First, I opened a terminal in the `blinky/phase4` directory. I installed `rustup` using Homebrew:

```
brew install rustup
```

Then I installed Rust:

```
rustup-init
source "$HOME/.cargo/env"
```

Then I installed the nightly toolchain release of Rust, which is required to do OS development with Rust.

```
rustup override set nightly
rustup component add rust-src
```

I verified that Rust installed properly:

```
rustc --version
```

```
rustc 1.64.0-nightly (4493a0f47 2022-08-02)
```

Then I installed `xargo` which is used to build the OS.

```
cargo install xargo
```

Then I ensured `xargo` installed properly:

```
xargo --version
```

```
xargo 0.3.26
cargo 1.64.0-nightly (85b500cca 2022-07-24)
```

Since I use Visual Studio Code as my editor, I installed [Rust Analyzer](https://rust-analyzer.github.io) to help with code completion.
I opened the file `blinky/phase4/src/lib.rs` and edited it to add the blink functionality, very similar to the C code from Phase 3.

```diff
#[no_mangle]
pub unsafe extern "C" fn kmain() {
-    // FIXME: STEP 1: Set GPIO Pin 16 as output.
+    // 18 is the lower bit for pin 16 (bits 20-18)
+    // | is bitwise OR operator
+    GPIO_FSEL1.write_volatile(GPIO_FSEL1.read_volatile() | (0b001 << 18));
+
-    // FIXME: STEP 2: Continuously set and clear GPIO 16.
+    // infinite loop
+    loop {
+        // set pin 16 (turn it on)
+        GPIO_SET0.write_volatile(GPIO_SET0.read_volatile() | (0b1 << 16));
+        spin_sleep_ms(500);
+
+        // clear pin 16 (turn it off)
+        GPIO_CLR0.write_volatile(GPIO_CLR0.read_volatile() | (0b1 << 16));
+        spin_sleep_ms(500);
+    }
}
```

However, I found that I needed to make some other changes since I updated Rust to the latest version:

1. The `asm!` macro has been merged into Rust stable, and therefore no longer needs enabled. It also defaults to `volatile`, so I removed that parameter. ([source](https://users.rust-lang.org/t/volatile-option-in-new-asm-macro/44289))
2. I renamed `compiler_builtins_lib` to `compiler_builtins`.
3. I removed `pointer_methods` from the feature array since it's now in Rust stable.
4. I added `rustc_private` to the feature array.
5. I set the Rust edition in the `blinky/phase4/Cargo.toml` as `edition = "2021"`.
6. I removed the `rlibc` dependency in the `blinky/phase4/Cargo.toml` file.
7. I changed the `panic_fmt` `lang` annotation to a `panic_handler` annotation in the `blink/phase4/lang_items.rs` file.

I've outlined the diffs for the files below:

`blinky/phase4/lib.rs`
```diff
-#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
+#![feature(compiler_builtins, lang_items, rustc_private)]

...

-        unsafe { asm!("nop" :::: "volatile"); }
+        unsafe { core::arch::asm!("nop"); }
```

`blinky/phase4/Cargo.toml`
```diff
[package]
name = "blinky"
version = "0.1.0"
+edition = "2021"

...

-[dependencies]
-rlibc = "*"
```

`blinky/phase4/src/lang_items.rs`
```diff
-#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop{} }
+#[panic_handler] pub extern fn panic_handler(_info: &core::panic::PanicInfo) -> ! { loop{} }
```

Then I was able to successfully build the project:

```
make
```

I copied the `blinky/phase4/build/blinky.bin` output file to the SD card, renaming to `kernel8.img`.

The light started blinking, which means the OS built in Rust worked!