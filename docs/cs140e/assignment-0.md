# Assignment 0

I began this project by following the steps located at the CS140e course [here](https://cs140e.sergio.bz/assignments/0-blinky/). All steps in this documentation are executed within the `cs140e/assignment-0` directory.

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