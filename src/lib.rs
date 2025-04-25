#![no_std]
#![no_main]
#![crate_type = "rlib"]
mod func;
#[macro_use]
extern crate sc;

#[unsafe(no_mangle)]
pub fn main() {
    func::hello();
    func::exit(0);
}
