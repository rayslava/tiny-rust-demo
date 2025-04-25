# ELF header for the binary.
    .section .text
    .global _start
    .balign 8

# Base address
    .equ BASE_ADDR, 0x00400000

ehdr:
    # Magic number
    .byte 0x7f
    .ascii "ELF"
    
    # File class, data encoding, version, padding
    .byte 2, 1, 1, 0
    .byte 0, 0, 0, 0, 0, 0, 0, 0
    
    # Type, machine, version
    .word 2                # e_type = executable
    .word 0x3e             # e_machine = x86-64
    .long 1                # e_version
    
    # Entry point address
    .quad entry            # e_entry
    
    # Program header offset
    .quad phdr - ehdr      # e_phoff
    
    # Section header offset
    .quad 0                # e_shoff
    
    # Flags
    .long 0                # e_flags
    
    # ELF header size
    .word ehdrsize         # e_ehsize
    
    # Program header entry size
    .word phdrsize         # e_phentsize
    
    # Number of program headers
    .word 1                # e_phnum
    
    # Section header fields (not used)
    .word 0, 0, 0          # e_sh*

ehdrsize = . - ehdr

phdr:
    # Program header
    .long 1                # p_type = loadable program segment
    .long 7                # p_flags = rwx
    .quad 0                # p_offset
    .quad ehdr, ehdr       # p_vaddr, p_paddr (relative to BASE_ADDR)
    .quad filesize         # p_filesz
    .quad filesize         # p_memsz
    .quad 0x1000           # p_align

phdrsize = . - phdr

_start:
    .incbin "payload.bin"

filesize = . - ehdr
