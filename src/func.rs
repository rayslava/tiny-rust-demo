pub fn exit(n: usize) -> ! {
    unsafe {
        syscall!(EXIT, n);
        core::hint::unreachable_unchecked()
    }
}

fn write(fd: usize, buf: &[u8]) {
    unsafe {
        syscall!(WRITE, fd, buf.as_ptr(), buf.len());
    }
}

pub fn puts(buf: &[u8]) {
    write(1, buf);
}

fn read(fd: usize, buf: &mut [u8], count: usize) -> usize {
    unsafe { syscall!(READ, fd, buf.as_ptr(), count) }
}

pub fn gets(buf: &mut [u8]) -> usize {
    let res = read(0, buf, buf.len());

    if res == 0 {
        return 0;
    }

    let idx = if res <= buf.len() {
        res - 1
    } else {
        buf.len() - 1
    };

    if let Some(slot) = buf.get_mut(idx) {
        *slot = 0;
    }

    res
}
