pub struct CmdArgs {
    serial_up: bool,
    macro_start: Option<std::os::raw::c_ulong>,
    macro_len: Option<std::os::raw::c_ulong>,
    cnc_ip: String,
}

impl CmdArgs {
    pub fn get_args() -> Result<CmdArgs, Box<dyn std::error::Error>> {
        let mut ca = CmdArgs {
            serial_up: true,
            macro_start: None,
            macro_len: None,
            cnc_ip: String::from("127.0.0.1"),
        };
        let mut args = std::env::args().skip(1); // Skip the program name
    
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--nonserial" => ca.serial_up = false,
                "-s" => {
                    ca.serial_up = false;
                    match args.next() {
                        Some(a) => ca.macro_start = Some(a.parse()?),
                        None => return Err("usage: -s <start custom macro variable number>".into()),
                    }
                },
                "-l" => {
                    ca.serial_up = false;
                    match args.next() {
                        Some(a) => ca.macro_len = Some(a.parse()?),
                        None => return  Err("usage: -l <number of custom macro variable>".into()),
                    }
                }
                "--cncip" => {
                    match args.next() {
                        Some(a) => ca.cnc_ip = a,
                        None => return Err("usage: --cncip <CNC IP addrees>".into()),
                    }
                }
                _ => return Err("invalid argument".into()),
            }
        }
    
        Ok(ca)
    }

    pub fn get_serial_up(&self) -> bool {
        self.serial_up
    }

    pub fn get_macro_start(&self) -> Option<std::os::raw::c_ulong> {
        self.macro_start
    }

    pub fn get_macro_len(&self) -> Option<std::os::raw::c_ulong> {
        self.macro_len
    }
    pub fn get_cnc_ip(&self) -> &str {
        &self.cnc_ip
    }
}