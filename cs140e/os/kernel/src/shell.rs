use core::fmt::Write;

use pi::uart::MiniUart;
use stack_vec::StackVec;
use console::{CONSOLE, kprint, kprintln};

macro_rules! printtest {
    ($expression:expr) => ({
       //use core::fmt::Write;
        //let mut uart = pi::uart::MiniUart::new();
        //uart.write_str($expression);
    });
}

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) {
    MiniUart::new().write_str("Debug 1").unwrap();
    MiniUart::new().write_fmt(format_args!("{}", "Debug 2")).unwrap();
    /*loop {
        printtest!("1\n");
        kprint!("{}", prefix);
        printtest!("2\n");
        let mut storage = [0u8; 512];
        let mut input = StackVec::new(&mut storage);
        printtest!("3\n");
        //while let byte = console.read_byte() {
        loop {
            printtest!("4\n");
            let byte = CONSOLE.lock().read_byte();
            printtest!("5\n");
            kprint!("{}", byte as char);
            printtest!("6\n");
            if byte == 0x8 || byte == 0x7f {
                if !input.is_empty() {
                    input.pop();
                    kprint!("{} {}", 0x8, 0x8);
                }
            } else if byte == b'\r' || byte == b'\n' {
                kprintln!();
                let mut args = [""; 64];

                match Command::parse(core::str::from_utf8(&input).unwrap(), &mut args) {
                    Ok(command) => match command.path() {
                        "echo" => {
                            for arg in command.args.iter().skip(1) {
                                kprint!("{} ", arg);
                            }
                            kprintln!();
                        },
                        path => {
                            kprintln!("unknown command: {}", path);
                        }
                    },
                    Err(Error::Empty) => (),
                    Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
                }
                break;
            } else if byte.is_ascii() {
                match input.push(byte) {
                    Ok(_) => (),
                    Err(_) => kprintln!("error: input too long"),
                }
            } else {
                kprint!("{}", 0x7); // beep, invalid character
            }
        }
    }*/
}
