mod cncadapter;
mod cmd_args;

use cncadapter::CncAdapter;
use serialport::{DataBits, Parity, StopBits};
use cmd_args::CmdArgs;
use std::fs::write;

const SERIAL_PORT: &str = "/dev/ttyUSB0";
// const CNC_IP: &str = "192.168.1.10";
const CNC_PORT: std::os::raw::c_ushort = 8193;
const OFILE: &str = "temp.txt";
const MFILE: &str = "macro.txt";

fn macror_to_file<T, I, L>(macror: I, filepath: T ,len: L, y:f64) -> Result<String, Box<dyn std::error::Error>>
where
    I: IntoIterator,
    I::Item: std::fmt::Display,
    T: AsRef<std::path::Path>,
    L: std::fmt::Display,
{
    let m: String = macror.into_iter().map(|f| format!("{}\n", f)).collect();
    let macror_len = m + &format!("X{}\n", len);
    let all = macror_len + &format!("Y{}", y);
    write(filepath, &all)?;
    Ok(all)
}

fn main() {
    let mut serial_buf: Vec<u8> = vec![0; 4096];

    let args = CmdArgs::get_args()
        .expect("usage:\n\t-s <start custom macro variable number>\n\t-l <number of custom macro variable>");

    let mut cnc = CncAdapter::new(args.get_cnc_ip(), CNC_PORT);
    cnc.connect_cnc().unwrap();

    if args.get_serial_up() {
        let mut serial_reading: bool = true;
        let mut port = serialport::new(SERIAL_PORT, 4800)
            .data_bits(DataBits::Seven)
            .stop_bits(StopBits::Two)
            .parity(Parity::Even)
            .timeout(std::time::Duration::from_millis(500))
            .open().expect(&format!("Failed to open port {}", SERIAL_PORT));
        loop {
            while serial_reading {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(bytes_recvd) => {
                        serial_reading = write(OFILE, &serial_buf[..bytes_recvd]).is_err();
                    },
                    Err(..) => eprintln!("Found no data!"),
                }
            }
            serial_reading = true;
            match cnc.download_file(OFILE) {
                Ok(o) => println!("{}", o),
                Err(e) => eprintln!("{}", e),
            }
        }
    } else {
        match args.get_macro_start() {
            Some(s) => {
                match args.get_macro_len() {
                    Some(l) => {
                        let macror = cnc.read_macro(s, l).unwrap();
                        println!("{:?}", macror);
                        macror_to_file(macror, MFILE, l, cnc.read_macro(503, 1).unwrap()[0]).unwrap();
                    },
                    None => (),
                }
            },
            None => println!("{}", cnc.download_file(OFILE).unwrap()),
        }
    }
}