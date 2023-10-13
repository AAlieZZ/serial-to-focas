mod cncadapter;

use cncadapter::CncAdapter;
// use serialport::{self, DataBits, Parity, StopBits};

// const SERIAL_PORT: &str = "/dev/ttyUSB0";
const CNC_IP: &str = "192.168.1.10";
const CNC_PORT: std::os::raw::c_ushort = 8193;
const OFILE: &str = "temp.txt";
pub const EW_OK: i16 = cncadapter::fwlib32::EW_OK as i16;
pub const EW_BUFFER: i16 = cncadapter::fwlib32::EW_BUFFER as i16;

fn main() {
    // let mut port = serialport::new(SERIAL_PORT, 4800)
    //     .data_bits(DataBits::Seven)
    //     .stop_bits(StopBits::Two)
    //     .parity(Parity::Even)
    //     .timeout(std::time::Duration::from_millis(500))
    //     .open().expect("Failed to open port");
    
    // let mut serial_buf: Vec<u8> = vec![0; 32];
    // port.read(serial_buf.as_mut_slice()).expect("Found no data!");

    let mut cnc = CncAdapter::new(CNC_IP, CNC_PORT);
    match cnc.connect_cnc() {
        Ok(o) => println!("{}", o),
        Err(e) => println!("{}", e),
    }
    cnc.download_file(OFILE).unwrap();
}