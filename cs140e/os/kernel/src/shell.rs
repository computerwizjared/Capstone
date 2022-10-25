use console::{kprint, kprintln, CONSOLE};
use stack_vec::StackVec;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
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
    loop {
        kprint!("{}", prefix);
        let mut storage = [0u8; 512];
        let mut input = StackVec::new(&mut storage);
        loop {
            let byte = CONSOLE.lock().read_byte();
            kprint!("{}", byte as char);
            if byte == 0x8 || byte == 0x7f {
                if !input.is_empty() {
                    input.pop();
                    kprint!("{} {}", 0x8 as char, 0x8 as char);
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
                        }
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
    }
}
