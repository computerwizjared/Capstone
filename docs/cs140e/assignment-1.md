# Assignment 1

I began this project by following the steps located at the CS140e course [here](https://cs140e.sergio.bz/assignments/1-shell/). All steps in this documentation are executed within the `cs140e/assignment-1` directory.

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