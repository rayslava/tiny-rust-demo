# Minimal static Rust app template

The template is based on the original
[work](https://mainisusuallyafunction.blogspot.com/2015/01/151-byte-static-linux-binary-in-rust.html) by Keegan McAllister.

The source code has been updated to work with stable Rust compiler and not to
require the `nasm` installation. Now it basically should work with
`>=rustc-1.86` without nightly or unstable features.

# Usage

Just run `./build.sh` which will provide a 352-bytes "Hello, user" program for
x86_64 Linux.

# Purpose

This template is intended to be a base for tiny apps for embedding: `initramfs`
or other limited environments.
