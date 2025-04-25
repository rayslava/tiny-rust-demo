#!/bin/bash

set -e

cargo clean
rm -f *.o *.bin payload tinyrust

cargo build --release

ld --gc-sections -e main -T script.ld -o payload $(find target/release -name '*.rlib')
objcopy -j combined -O binary payload payload.bin

ENTRY=$(nm -f posix payload | grep '^main ' | awk '{print $3}')
as -o elf.o elf.S
ld --oformat=binary -o tinyrust -Ttext=0x400000 --defsym=entry=0x$ENTRY elf.o

chmod +x tinyrust
wc -c tinyrust
