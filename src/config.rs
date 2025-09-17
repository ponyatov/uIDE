//! shared config

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/// VM configuration parameters
pub mod VM {
    /// max VM memory size
    pub const Msz: usize = 0x10000;
    /// return stack size (max call depth)
    pub const Rsz: usize = 0x100;
    /// data stack size (limited for MCU use)
    pub const Dsz: usize = 0x10;
}

/// Web server default bind ip
pub mod server {
    pub const ip: &str = "127.0.0.1";
    // pub const IP: &str = "0.0.0.0";
    /// Web server IP port
    pub const port: u16 = 12345;
    /// bind address constant
    pub const bind: &str = const_format::formatcp!("{ip}:{port}");
}
