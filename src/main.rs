use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    // Enable raw mode
    enable_raw_mode()?;

    // Clear screen and hide cursor
    print!("\x1b[2J\x1b[?25l");
    stdout.flush()?;

    let mut x: i32 = 10;
    let mut y: i32 = 10;

    // Main loop
    loop {
        // Move cursor to position and show a blinking block
        print!("\x1b[{};{}H█", y, x);
        stdout.flush()?;

        // Read a single byte
        let mut buf = [0u8; 1];
        if stdin.read(&mut buf)? == 0 {
            // if stdin is closed, this allows us to close gracefully?
            break;
        }

        // Clear the current cursor position before drawing in new location
        print!("\x1b[{};{}H ", y, x);

        // Check for escape sequences (arrow keys)
        if buf[0] == 27 {
            // Read the next two bytes for arrow keys
            let mut seq = [0u8; 2];
            stdin.read(&mut seq)?;

            if seq[0] == b'[' {
                match seq[1] {
                    b'A' => y = y.saturating_sub(1).max(1), // Up
                    b'B' => y = (y + 1).min(24),            // Down
                    b'C' => x = (x + 1).min(80),            // Right
                    b'D' => x = x.saturating_sub(1).max(1), // Left
                    _ => {}
                }
            }
        } else if buf[0] == b'q' || buf[0] == 3 {
            // Quit on 'q' or Ctrl+C (on mac, ctrl+c doesn't trigger,, maybe on windows?)
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    print!("\x1b[2J\x1b[H\x1b[?25h");
    print!("KTHXBYE\n");
    stdout.flush()?;

    Ok(())
}

fn enable_raw_mode() -> io::Result<()> {
    #[cfg(any(unix, target_family = "wasm"))]
    {
        use std::os::unix::io::AsRawFd;
        let fd = io::stdin().as_raw_fd();

        let mut termios = std::mem::MaybeUninit::<libc::termios>::uninit();
        unsafe {
            if libc::tcgetattr(fd, termios.as_mut_ptr()) != 0 {
                return Err(io::Error::last_os_error());
            }
            let mut termios = termios.assume_init();

            // Disable canonical mode and echo
            termios.c_lflag &= !(libc::ICANON | libc::ECHO);

            // Set minimum characters to return and timeout
            termios.c_cc[libc::VMIN] = 1;
            termios.c_cc[libc::VTIME] = 0;

            if libc::tcsetattr(fd, libc::TCSANOW, &termios) != 0 {
                return Err(io::Error::last_os_error());
            }
        }
    }
    Ok(())
}

fn disable_raw_mode() -> io::Result<()> {
    #[cfg(any(unix, target_family = "wasm"))]
    {
        use std::os::unix::io::AsRawFd;
        let fd = io::stdin().as_raw_fd();

        let mut termios = std::mem::MaybeUninit::<libc::termios>::uninit();
        unsafe {
            if libc::tcgetattr(fd, termios.as_mut_ptr()) != 0 {
                return Err(io::Error::last_os_error());
            }
            let mut termios = termios.assume_init();

            // Re-enable canonical mode and echo
            termios.c_lflag |= libc::ICANON | libc::ECHO;

            if libc::tcsetattr(fd, libc::TCSANOW, &termios) != 0 {
                return Err(io::Error::last_os_error());
            }
        }
    }
    Ok(())
}
