extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use] extern crate structopt_derive;

use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};
use xmodem::{Xmodem, Progress};

mod parsers;

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader, BufRead};

    let opt = Opt::from_args();
    let mut serial = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    let mut settings = serial.read_settings().expect("failed to read settings");
    settings.set_baud_rate(opt.baud_rate).expect("failed to set baud rate");
    settings.set_char_size(opt.char_width);
    settings.set_stop_bits(opt.stop_bits);
    settings.set_flow_control(opt.flow_control);

    serial.write_settings(&settings).expect("failed to write settings");
    serial.set_timeout(Duration::from_secs(opt.timeout)).expect("failed to set timeout");

    if opt.raw {
        match opt.input {
            Some(path) => {
                let mut file = File::open(path).expect("failed to open file");
                io::copy(&mut file, &mut serial).expect("failed to copy file to tty");
            }
            None => {
                io::copy(&mut io::stdin(), &mut serial).expect("failed to copy input to tty");
            }
        }
    } else {
        match opt.input {
            Some(path) => {
                let file = File::open(path).expect("failed to open file");
                match Xmodem::transmit_with_progress(file, &mut serial, progress_fn) {
                    Ok(_) => {},
                    Err(error) => panic!("error occurred during transmission: {:?}", error)
                }
            }
            None => {
                match Xmodem::transmit_with_progress(io::stdin(), &mut serial, progress_fn) {
                    Ok(_) => {},
                    Err(error) => panic!("error occurred during transmission: {:?}", error)
                }
            }
        }
    }
}
