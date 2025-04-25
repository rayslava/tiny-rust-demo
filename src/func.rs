pub fn exit(n: usize) -> ! {
    unsafe {
        syscall!(EXIT, n);
        core::hint::unreachable_unchecked()
    }
}

pub fn write(fd: usize, buf: &[u8]) {
    unsafe {
        syscall!(WRITE, fd, buf.as_ptr(), buf.len());
    }
}

pub fn puts(buf: &[u8]) {
    write(1, buf);
}

pub fn read(fd: usize, buf: &mut [u8], count: isize) {
    unsafe {
        syscall!(READ, fd, buf.as_ptr(), count);
    }
}

pub fn gets(buf: &mut [u8]) {
    read(0, buf, buf.len() as isize);
}

pub fn hello() {
    puts(b"Hello!\n");
}
