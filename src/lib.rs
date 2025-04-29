#![no_std]
#![no_main]
#![crate_type = "rlib"]
#[macro_use]
extern crate sc;

mod editor;
mod syscall;
mod terminal;
mod termios;

use editor::run_editor;
use syscall::{exit, putchar, puts};
use terminal::{get_termios, get_winsize, set_raw_mode, set_termios};
use termios::{Termios, Winsize, TCSETS, TCSETSW};

// Main function
#[no_mangle]
pub fn main() -> ! {
    puts(b"BASic EDitor v0.1\r\n");

    // Get window size
    let mut winsize = Winsize::new();

    if get_winsize(syscall::STDOUT, &mut winsize).is_ok() {
        puts(b"Terminal size: ");
        terminal::write_number(winsize.rows);
        puts(b"x");
        terminal::write_number(winsize.cols);
        puts(b"\r\n");
    } else {
        puts(b"Could not get terminal size\r\n");
    }

    // Get and save original terminal settings
    let mut orig_termios = Termios::new();

    if get_termios(syscall::STDIN, &mut orig_termios).is_ok() {
        // Make a copy for raw mode
        let mut raw_termios = orig_termios;

        // Set raw mode flags
        set_raw_mode(&mut raw_termios);

        // Apply raw mode
        if set_termios(syscall::STDIN, TCSETS, &raw_termios).is_ok() {
            puts(b"Entered raw mode. Press q to exit.\r\n");

            // Run the editor
            run_editor();

            // Restore original settings
            set_termios(syscall::STDIN, TCSETSW, &orig_termios);
            puts(b"\r\nExited raw mode\r\n");
        } else {
            puts(b"Failed to set raw mode\r\n");
        }
    } else {
        puts(b"Failed to get terminal attributes\r\n");
    }

    exit(0);
}
