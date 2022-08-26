SECTIONS
{
    /* OS startup code in first section */
    .text.boot : { *(.text.boot) }
    /* kernel-compiled instructions */
    .text :  { *(.text) }
    /* read-only data */
    .rodata : { *(.rodata) }
    /* normal data */
    .data : { *(.data) }
    /* bss is data that is initialized to 0 */
    /* align to 8-byte array since it is a string */
    . = ALIGN(0x8);
    bss_begin = .;
    .bss : { *(.bss*) }
    bss_end = .;
}