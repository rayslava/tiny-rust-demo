SECTIONS {
    . = 0x400080;

    combined . : AT(0x400080) ALIGN(1) SUBALIGN(1) {
        *(.text*)
        *(.data*)
        *(.rodata*)
        *(.bss*)
    }
}
