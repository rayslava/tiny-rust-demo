#![allow(dead_code)]

// Termios constants
pub const TCGETS: usize = 0x5401;
pub const TCSETS: usize = 0x5402;
pub const TCSETSW: usize = 0x5403;
pub const TCSETSF: usize = 0x5404;
pub const TIOCGWINSZ: usize = 0x5413;

// Termios flag constants
pub const ECHO: u32 = 0o000010;
pub const ICANON: u32 = 0o000002;
pub const ISIG: u32 = 0o000001;
pub const IEXTEN: u32 = 0o100000;
pub const BRKINT: u32 = 0o000002;
pub const ICRNL: u32 = 0o000400;
pub const INPCK: u32 = 0o000020;
pub const ISTRIP: u32 = 0o000040;
pub const IXON: u32 = 0o002000;
pub const OPOST: u32 = 0o000001;
pub const CS8: u32 = 0o000060;

// Termios special character positions
pub const VMIN: usize = 6;
pub const VTIME: usize = 5;

// Window size structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Winsize {
    pub rows: u16,
    pub cols: u16,
    pub xpixel: u16,
    pub ypixel: u16,
}

impl Winsize {
    pub fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            xpixel: 0,
            ypixel: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        unsafe { core::mem::transmute(self) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8; 8] {
        unsafe { core::mem::transmute(self) }
    }
}

// Termios structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Termios {
    pub iflag: u32,   // Input modes
    pub oflag: u32,   // Output modes
    pub cflag: u32,   // Control modes
    pub lflag: u32,   // Local modes
    pub line: u8,     // Line discipline
    pub cc: [u8; 32], // Control characters
    pub ispeed: u32,  // Input speed
    pub ospeed: u32,  // Output speed
}

impl Termios {
    pub fn new() -> Self {
        Self {
            iflag: 0,
            oflag: 0,
            cflag: 0,
            lflag: 0,
            line: 0,
            cc: [0; 32],
            ispeed: 0,
            ospeed: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8; 60] {
        unsafe { core::mem::transmute(self) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8; 60] {
        unsafe { core::mem::transmute(self) }
    }
}
