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

/// GUI configuration
pub mod GUI {
    /// main window
    pub mod WIN {
        /// title
        pub const TITLE: &str = "μIDE";
        /// default size
        pub const SIZE: (f64, f64) = (640.0, 480.0);
    }
    /// code font
    pub mod Font {
        pub const FAMILY: &str = "Monospace";
        pub const SIZE: f64 = 14.0;
    }
    /// theme colors
    pub mod Color {
        /// background
        pub const Background: druid::Color = druid::Color::rgb8(0x22, 0x22, 0x22);
        /// status/menu line background
        pub const Status: druid::Color = druid::Color::rgb8(0x11, 0x11, 0x11);
        /// generic text
        pub const Text: druid::Color = druid::Color::rgb8(0xF8, 0xF8, 0xF2);
    }
}
