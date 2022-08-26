#include "mm.h"

.section ".text.boot"       // tell the linker that this is the boot section

.globl _start               // global function to start the kernel
_start:
    MRS     X0, mpidr_el1   // copy processor register mpdir_el1 (where the processor id is) to system register
    AND     X0, X0, #0xFF   // compare X0 to 11111111, so if not 00000000, X0 will be populated with a non-0 value
    CBZ     X0, main        // if X0 is all 0 jump to main, otherwise continue
    B       hang            // branch off to a hang for all non-primary cpus

hang:
    B       hang            // infinite branch loop to hang processor

main:
    ADR     X0, bss_begin   // load start address of bss into X0
    ADR     X1, bss_end     // load end address of bss into X1
    SUB     X1, X1, X0      // since memzero function only takes start address and size, subtract end (X1) from start (X0) to get size and load into X1
    BL      memzero         // branch with a link (stored in X30) to memzero function

    MOV     sp, #LOW_MEMORY // initialize stack pointer to immediately follow kernel stack
    BL      kernel_main     // branch with a link (stored in X30) to main kernel function