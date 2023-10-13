pub mod fwlib32;

use fwlib32::EW_OK;
use crate::cncadapter::fwlib32::cnc_dwnend4;

const LOG_PATH: &str = "/var/log/foces32.log";

pub struct CncAdapter<'a> {
    m_flib_hndl: std::os::raw::c_ushort,
    m_connect_ok: bool,
    // m_cnc_start: bool,
    m_port: std::os::raw::c_ushort,
    m_ip: &'a str,
}

impl CncAdapter<'_> {
    pub fn new(ip: &str, port: std::os::raw::c_ushort) -> CncAdapter {
        CncAdapter {
            m_flib_hndl: 0,
            m_connect_ok: false,
            // m_cnc_start: false,
            m_ip: ip,
            m_port: port,
        }
    }

    pub fn connect_cnc(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        if unsafe {fwlib32::cnc_startupprocess(0, LOG_PATH.as_ptr() as *const i8)} != EW_OK as i16 {
            return Err("Statup focas proceess fail!".into());
        }
        else {
            println!("Statup focas proceess success.\n");
        }
        self.m_connect_ok = false;
        self.disconnect_cnc().unwrap();
        let ip = self.m_ip.as_ptr() as *const i8;
        let handle = &mut self.m_flib_hndl as *mut u16;
        unsafe {
            if fwlib32::cnc_allclibhndl3(ip, self.m_port, 3, handle) != EW_OK as i16 {
                return Err(format!("CNC connect ...... Fail. ip={}, port={}, handle={}", self.m_ip, self.m_port, self.m_flib_hndl).into());
            }
        }
        self.m_connect_ok = true;
        Ok(format!("CNC connect ...... OK. ip={}, port={}\nCNC focas handler = {}", self.m_ip, self.m_port, self.m_flib_hndl))
    }

    pub fn disconnect_cnc(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.m_flib_hndl > 0 {
            unsafe {
                if fwlib32::cnc_freelibhndl(self.m_flib_hndl) == EW_OK as i16 {
                    println!("Disconnect from cnc success.");
                }
            }
            self.m_flib_hndl = 0;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        self.m_connect_ok = false;
        Ok(())
    }

    pub fn download_file<T: AsRef<std::path::Path>>(&self, filepath: T) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = std::fs::read_to_string(filepath)?;
        let cnc_path = String::from("//CNC_MEM/USER/PATH1/").as_mut_ptr() as *mut i8;
        unsafe {
            let ret = fwlib32::cnc_dwnstart4(self.m_flib_hndl, 0, cnc_path);
            if ret != EW_OK as i16 {
                return Err(format!("CncAdapter::download_file(): cnc_dwnstart4 error. handler={}, error={}", self.m_flib_hndl, ret).into());
            }
        }
        println!("CncAdapter::download_file(): cnc_dwnstart4 ......  success");
        let mut len = buf.len() as std::os::raw::c_long;
        let mut n: std::os::raw::c_long = 0;
        let mut prg = buf.as_mut_ptr() as *mut i8;
        while len > 0 {
            println!("CncAdapter::download_file(): cnc_download4 ...... left size = {}", len);
            n = len;
            match unsafe {fwlib32::cnc_download4(self.m_flib_hndl, &mut n as *mut std::os::raw::c_long, prg)} {
                crate::EW_BUFFER => continue,
                crate::EW_OK => {
                    prg = unsafe {prg.add(n as usize)};
                    len -= n;
                },
                _ => break,
            }
        }
        if len > 0 {
            let mut ncn_err = fwlib32::ODBERR{
                err_no: 0,
                err_dtno: 0,
            };
            unsafe {
                fwlib32::cnc_getdtailerr(self.m_flib_hndl, &mut ncn_err as *mut fwlib32::ODBERR);
            }
            return Err(format!("CncAdapter::download_file(): cnc_download4 error. err_no={}, err_dtno={}",&ncn_err.err_no, &ncn_err.err_dtno).into());
        }
        if unsafe {
            cnc_dwnend4(self.m_flib_hndl)
        } != EW_OK as i16 {
            let mut ncn_err = fwlib32::ODBERR{
                err_no: 0,
                err_dtno: 0,
            };
            unsafe {
                fwlib32::cnc_getdtailerr(self.m_flib_hndl, &mut ncn_err as *mut fwlib32::ODBERR);
            }
            if ncn_err.err_no == 4 {
                return Err(format!("CncAdapter::download_file(): cnc_dwnend4 error. err_no={}, err_dtno={}\nCncAdapter::downloadfile(): cnc_dwnend4 error.The same NC program has already been registered!",ncn_err.err_no, ncn_err.err_dtno).into());
            }
            else {
                return  Err(format!("CncAdapter::downloadFil(): cnc_dwnend4 error. err_no={}, err_dtno={}",ncn_err.err_no, ncn_err.err_dtno).into());
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        Ok(String::from("CncAdapter::downloadFil(): download nc file success."))
    }
}
