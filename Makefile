# set a GCC prefix to enable cross-compilation between architectures
# Raspberry Pi 3/4 is aarch64
CROSS_COMPILER_PREFIX ?= aarch64-none-elf

# -Wall shows all warnings
# -nostdlib disables the C standard library, since this will be running on bare metal
# -nostartfiles disables the built-in startup files that set initial stack pointer, initialize static data, and jumping to main entrypoint
# -ffreestanding tells the compiler not to assume that standard functions exist
# -Iinclude searches for headers in the "include" directory
# -mgeneral-regs-only tells the compiler to only use the general purpose registers for simplicity
# C_OPTIONS = -Wall -nostdlib -nostartfiles -ffreestanding -Iinclude -mgeneral-regs-only
ASSEMBLY_OPTIONS = -Iinclude

# set the output build directory
BUILD_DIRECTORY = target
# set the input source directory
SOURCE_DIRECTORY = src

# set the default make target
all: kernel8.img

# deletes all files in the build directory
clean:
	rm -rf $(BUILD_DIRECTORY) *.img

# compiles Rust files
rust:
	xargo build --target $(CROSS_COMPILER_PREFIX)

# compiles assembly files
$(BUILD_DIRECTORY)/%_S.o: $(SOURCE_DIRECTORY)/%.S
	$(CROSS_COMPILER_PREFIX)-gcc $(ASSEMBLY_OPTIONS) -MMD -c $< -o $@

# create arrays of compiled output files
RUST_FILES = $(wildcard $(BUILD_DIRECTORY)/$(CROSS_COMPILER_PREFIX)/debug/*.rlib)
ASSEMBLY_FILES = $(wildcard $(SOURCE_DIRECTORY)/*.S)
OBJECT_FILES = $(RUST_FILES:$(BUILD_DIRECTORY)/$(CROSS_COMPILER_PREFIX)/debug/.rlib=$(BUILD_DIRECTORY)/%_rlib.o)
OBJECT_FILES += $(ASSEMBLY_FILES:$(SOURCE_DIRECTORY)/%.S=$(BUILD_DIRECTORY)/%_S.o)

# include all compiled dependency files
DEPENDENCY_FILES = $(OBJECT_FILES:%.o=%.d)
-include $(DEPENDENCY_FILES)

# build kernel from compiled object files
# elf files are for an OS to execute, not to run bare-metal on hardware
# extract elf file and create img from it, 8 signifies ARMv8 (64-bit)
kernel8.img: $(SOURCE_DIRECTORY)/linker.ld $(OBJECT_FILES)
	$(CROSS_COMPILER_PREFIX)-ld -T $(SOURCE_DIRECTORY)/linker.ld -o $(BUILD_DIRECTORY)/kernel8.elf -ffreestanding -O2 -nostdlib $(OBJECT_FILES)
	$(CROSS_COMPILER_PREFIX)-objcopy $(BUILD_DIRECTORY)/kernel8.elf -O binary kernel8.img