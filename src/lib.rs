#![no_std]
#![no_main]
#![crate_type = "rlib"]
mod func;
#[macro_use]
extern crate sc;

use crate::func::{exit, gets, puts};

pub fn hello() {
    let mut buf = [0; 64];
    puts(b"Who are you?\n");
    gets(&mut buf);
    puts(b"Hello, ");
    puts(&buf);
    puts(b".\n");
}

#[unsafe(no_mangle)]
pub fn main() -> ! {
    hello();
    exit(0);
}
