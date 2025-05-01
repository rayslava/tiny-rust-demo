use crate::syscall::{STDIN, STDOUT, putchar, puts, read};

// Function to read one character
fn read_char() -> Option<u8> {
    let mut buf = [0u8; 1];
    match read(STDIN, &mut buf, 1) {
        Ok(n) if n > 0 => Some(buf[0]),
        _ => None,
    }
}

// Define key types
enum Key {
    Char(u8),
    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowLeft,
    Enter,
    Backspace,
    Quit,
}

// Read a key, handling escape sequences and control characters
fn read_key() -> Option<Key> {
    if let Some(ch) = read_char() {
        match ch {
            // Quit key
            b'q' => Some(Key::Quit),

            // Enter key
            b'\r' => Some(Key::Enter),

            // Backspace
            127 | 8 => Some(Key::Backspace),

            // Escape sequence
            27 => {
                // Detect arrow keys
                if let Some(b'[') = read_char() {
                    if let Some(ch) = read_char() {
                        match ch {
                            b'A' => return Some(Key::ArrowUp),
                            b'B' => return Some(Key::ArrowDown),
                            b'C' => return Some(Key::ArrowRight),
                            b'D' => return Some(Key::ArrowLeft),
                            _ => return Some(Key::Char(ch)),
                        }
                    }
                }
                Some(Key::Char(ch))
            }

            // Emacs key bindings - Control characters
            2 => Some(Key::ArrowLeft),  // C-b (backward-char)
            6 => Some(Key::ArrowRight), // C-f (forward-char)
            14 => Some(Key::ArrowDown), // C-n (next-line)
            16 => Some(Key::ArrowUp),   // C-p (previous-line)

            // Regular character
            _ => Some(Key::Char(ch)),
        }
    } else {
        None
    }
}

// Simple editor implementation
pub fn run_editor() {
    let mut running = true;

    // Track cursor position
    let mut cursor_row = 0;
    let mut cursor_col = 0;

    while running {
        if let Some(key) = read_key() {
            match key {
                Key::Quit => running = false,

                Key::Enter => {
                    let _ = puts(b"\r\n");
                    cursor_row += 1;
                    cursor_col = 0;
                }

                Key::Backspace => {
                    if cursor_col > 0 {
                        let _ = puts(b"\x08 \x08");
                        cursor_col -= 1;
                    }
                }

                Key::ArrowUp => {
                    if cursor_row > 0 {
                        let _ = puts(b"\x1b[A"); // Move cursor up
                        cursor_row -= 1;
                    }
                }

                Key::ArrowDown => {
                    let _ = puts(b"\x1b[B"); // Move cursor down
                    cursor_row += 1;
                }

                Key::ArrowRight => {
                    let _ = puts(b"\x1b[C"); // Move cursor right
                    cursor_col += 1;
                }

                Key::ArrowLeft => {
                    if cursor_col > 0 {
                        let _ = puts(b"\x1b[D"); // Move cursor left
                        cursor_col -= 1;
                    }
                }

                Key::Char(ch) => {
                    let _ = putchar(ch);
                    cursor_col += 1;
                }
            }
        }
    }
}
