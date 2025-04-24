#![no_main]
#![feature(core_intrinsics)]
#![crate_type = "rlib"]
#![allow(unstable)]
#[macro_use]
extern crate sc;

use std::intrinsics;

fn exit(n: usize) -> ! {
    unsafe {
        syscall!(EXIT, n);
        intrinsics::unreachable()
    }
}

fn write(fd: usize, buf: &[u8]) {
    unsafe {
        syscall!(WRITE, fd, buf.as_ptr(), buf.len());
    }
}

#[unsafe(no_mangle)]
pub fn main() {
    write(1, "Hello t!\n".as_bytes());
    exit(0);
}
