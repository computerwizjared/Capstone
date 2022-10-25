# Assignment 1

I began this project by following the steps located at the CS140e course [here](https://cs140e.sergio.bz/assignments/1-shell/). All steps from Phase 0 and Phase 1 of this documentation are executed within the `cs140e/assignment-1` directory.

## Phase 0

First, I installed `socat`:

```
brew install socat
```

Then I cloned the "shell" skeleton project from the CS140e Git repository into this repository under the `shell` folder.

```
git clone https://cs140e.sergio.bz/assignments/1-shell/skeleton.git shell
rm -rf shell/.git
```

Then, I answered the question in the `shell/getting-started/questions/assignment0` file.

## Phase 1

I ran the test script to check that it worked:

```
cd shell/ferris-wheel
./test.sh -v
```

And I encountered some errors related to updating my Rust version:

```
---------------------- stderr --------------------------
error: unknown unstable option: `no-trans`
```

To fix this, I edited the `test.sh` and renamed the `no-trans` option to `no-codegen` ([source](https://github.com/flycheck/flycheck-rust/issues/66#issuecomment-419397291)).

```diff
  if [ -n "${FILTER}" ] && ! [[ "${filename}" == *"${FILTER}"* ]]; then
    verbose -e "${KBLU}SKIPPING: ${KWHT}${filename}${KNRM}"
    return 0
  fi

-  stderr=$(rustc "${file}" $RUSTC_FLAGS -Z no-trans 2>&1)
+  stderr=$(rustc "${file}" $RUSTC_FLAGS -Z no-codegen 2>&1)
  result=$?
```

This results in more expected results. However, all the tests are supposed to fail, but due to Rust adding more features over the years, 2 passed automatically.

```
2 passes, 23 failures
```

I then went through and fixed the various failing tests and documented my changes in the `questions` folder.

## Phase 2

First, I answered the questions.
Then, I implemented StackVec in `shell/stack-vec/src/lib.rs` and ran the tests from the `shell/stack-vec` directory with the command `cargo test`.

I used these resources to help me implement StackVec:
- https://doc.rust-lang.org/std/ops/trait.Deref.html
- https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
- https://learning-rust.github.io/docs/e2.panicking.html
- https://doc.rust-lang.org/std/macro.panic.html

Then, I implemented xmodem in `shell/xmodem/src/lib.rs` and ran the tests from the `shell/xmodem` directory with the command `cargo test`.

I used these resources to help me implement xmodem:
- https://users.rust-lang.org/t/how-to-store-function-pointers-in-struct-and-call-them/51348
- https://www.reddit.com/r/rust/comments/enexin/accessing_len_while_iterating_over_mutable_vector/
- https://stackoverflow.com/a/38896376/5991792

Then, I implemented ttywrite in `shell/ttywrite/src/main.rs` and ran the tests from the `shell/ttywrite` directory with the command `./test.sh`. I got this error:

```
error writing output file
/usr/bin/base64: I/O error on input
```

So I had to edit `test.sh`:

```diff
function rand_string() {
-  base64 < /dev/urandom | head -c $((1 + RANDOM % 512))
+  cat /dev/urandom | head -c $((1 + RANDOM % 512)) | base64
}
```

I used this resource to help me implement ttywrite:
- https://doc.rust-lang.org/std/option/

## Phase 3

I cloned the `os` repository to :

```
git clone https://cs140e.sergio.bz/os.git os
rm -rf os/.git
```

Then in the `os/kernel` directory, I ran `make` and saw this error:

```
error: no matching package named `std_unicode` found
```

I had to re-work some code to get it to compile because of this PR on Rust's repo: https://github.com/rust-lang/rust/pull/49698.

But in addition to that, I had to do a LOT of other work to get it to compile.
Since the Rust version I'm using is so much newer than the one the CS140e course is using, I had to find a way to update it.
The easiest way was to look at an [alternative std library](https://github.com/technocreatives/core2) that someone had developed that is much slimmer and simpler than the standard Rust one.
I swapped that std library out and made a lot of other changes to get it to compile.

Then I made changes to `os/pi/src/timer.rs` to implement the timer.
I copied my code from the blinky project and pasted it into `os/kernel/src/kmain.rs`, and made the appropriate adjustments.
I built and copied the kernel.img, as before, in Assignment 0, and renamed it to kernel8.img.
I plugged the SD card into my Pi, and it worked!
I honestly didn't think it would work with all the work I had to do to make things compile with the new version of Rust, but it did!

Next, I implemented the GPIO driver in `os/pi/src/gpio.rs`.
This was pretty easy, and I was able to get the LED to blink using the driver rather than directly accessing the registers unsafely.
I copied the image and ran it on my Pi, and it worked.

Next, I implemented the UART driver in `os/pi/src/uart.rs`.
I added code to output if the LED is on or off, and it worked.

After this, I implemented the shell in `os/kernel/src/console.rs`, `os/kernel/src/shell.rs`, and `os/kernel/src/kmain.rs`.
I ran into a lot of issues with the `write_fmt` function call that held me up for a long time, but then I found [someone else who had the same issue](https://gitter.im/cs140e-rust/Lobby?at=5ab1c572c3c5f8b90d96bfd4), which led me to the solution.
For reference, I placed the text from `@paulmeyer` below:

> paulmeyer @paulmeyer Mar 20 2018 21:37
> Interesting note: I was working on the bootloader and having some issues, and couldn't get the console to work (for printing debug messages). Debugged a bit, then went back to the previously working 'shell' kernel. That wouldn't print now either! Wasted a day debugging it, chasing it down into the write_fmt in the std_library hanging somewhere. I finally backed up to my working revision from 2 days ago, and that didn't work either! Gremlins. Finally figured it out: I had the bootloader config.txt on the SD card with the 0x4000000 location to load the binary (needed for the bootloader) The shell kernel still builds/links with 0x80000. Because most of the branches/loads are relative branches in assembly (don't care about the absolute location of the code), or explicitly memory mapped locations (GPIO, UART, etc.) lots of stuff worked perfectly. However, as soon as the code got to someplace it needed to do an absolute branch (likely off into some part of the std library that was far away, or some kind of branch table), it branched to 0x80xxx when my code lived at 0x4000xxx and started executing garbage, likely 0's, effectively a hang in this case. Make sure to switch the config.txt back if you go back to any of the old code!!

I realized I didn't even get the firmware according to the cs140e website at the beginning.
I had to download it from https://cs140e.sergio.bz/files/firmware.tar.gz and copy the config.txt to the SD card.
Once I did that, `write_fmt` started working properly!