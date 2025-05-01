.PHONY: rlib clean

all: based

rlib:
	cargo build --release

payload: rlib
	ld --gc-sections -e main -T script.ld -o payload $(shell find target/release -name '*.rlib')

payload.bin: payload
	objcopy -j combined -O binary payload $@

elf.S: payload.bin

elf.o: elf.S
	as -o elf.o elf.S

based: ENTRY = $(shell nm -f posix payload | grep '^main ' | awk '{print $$3}')
based: elf.o payload.bin
	ld --oformat=binary -o $@ --defsym=entry=0x$(ENTRY) -T result.ld elf.o
	chmod 755 $@

clean:
	cargo clean
	rm -f *.o *.bin payload payload.bin tinyrust based

wc: based
	wc -c based
