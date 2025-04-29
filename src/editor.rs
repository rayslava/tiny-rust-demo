use crate::syscall::{putchar, puts, read, STDIN, STDOUT};

fn read_char() -> Option<u8> {
    let mut buf = [0u8; 1];
    match read(STDIN, &mut buf, 1) {
        Ok(n) if n > 0 => Some(buf[0]),
        _ => None,
    }
}

pub fn run_editor() {
    let mut running = true;

    while running {
        if let Some(ch) = read_char() {
            match ch {
                b'q' => running = false,
                b'\r' => {
                    let _ = puts(b"\r\n");
                }
                // backspace
                127 | 8 => {
                    // Move cursor back, print space, move cursor back again
                    let _ = puts(b"\x08 \x08");
                }
                _ => {
                    let _ = putchar(ch);
                }
            }
        }
    }
}
