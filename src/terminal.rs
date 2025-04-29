use crate::syscall::{ioctl, putchar, SysResult};
use crate::termios::{
    Termios, Winsize, BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST,
    TCGETS, TCSETS, TIOCGWINSZ, VMIN, VTIME,
};

// Get window size
pub fn get_winsize(fd: usize, winsize: &mut Winsize) -> SysResult {
    ioctl(fd, TIOCGWINSZ, winsize.as_bytes_mut().as_mut_ptr() as usize)
}

// Get terminal attributes
pub fn get_termios(fd: usize, termios: &mut Termios) -> SysResult {
    ioctl(fd, TCGETS, termios.as_bytes_mut().as_mut_ptr() as usize)
}

// Set terminal attributes
pub fn set_termios(fd: usize, option: usize, termios: &Termios) -> SysResult {
    ioctl(fd, option, termios.as_bytes().as_ptr() as usize)
}

// Set raw mode flags
pub fn set_raw_mode(termios: &mut Termios) {
    // Input flags
    termios.iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);

    // Output flags
    termios.oflag &= !OPOST;

    // Control flags
    termios.cflag |= CS8;

    // Local flags
    termios.lflag &= !(ECHO | ICANON | ISIG | IEXTEN);

    // Control characters
    termios.cc[VMIN] = 1; // Return after 1 byte read
    termios.cc[VTIME] = 0; // No timeout
}

// Write a number
pub fn write_number(mut n: u16) {
    if n == 0 {
        let _ = putchar(b'0');
        return;
    }

    let mut digits = [0u8; 5];
    let mut i = 0;

    while n > 0 && i < 5 {
        digits[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        let _ = putchar(digits[i]);
    }
}
