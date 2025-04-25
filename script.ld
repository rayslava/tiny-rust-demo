SECTIONS {
    . = 0x400078;

    combined . : AT(0x400078) ALIGN(1) SUBALIGN(1) {
    KEEP(*(.text.main))     /* Ensure `main` is kept */
        *(.text*)
        *(.data*)
        *(.rodata*)
        *(.bss*)
    }
}
