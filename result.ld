SECTIONS {
    . = 0x400000;

    .text : {
        *(.text*)
    }

    .data : {
        *(.data*)
    }

    .rodata : {
        *(.rodata*)
    }

    .bss : {
        *(.bss*)
    }

    /DISCARD/ : { *(.note.gnu.property) *(.comment) *(.note*) }
}
