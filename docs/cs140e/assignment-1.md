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

I also had to re-work a lot of other code due to features becoming stable in Rust and no longer needing to be enabled.