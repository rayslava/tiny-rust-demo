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
    if res > 0 && res < buf.len() {
        buf[res - 1] = 0;
    } else {
        buf[buf.len() - 1] = 0;
    }
    res
}
