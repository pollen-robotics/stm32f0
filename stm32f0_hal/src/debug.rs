#[cfg(feature = "serial_panic")]
use core::fmt::Write;

#[lang = "panic_fmt"]
#[allow(unused_variables)]
#[no_mangle]
pub unsafe extern "C" fn panic_fmt(
    msg: ::core::fmt::Arguments,
    file: &'static str,
    line: u32,
    column: u32,
) -> ! {
    #[cfg(feature = "serial_panic")]
    {
        if let Some(ref mut log) = serial_panic::LOG {
            writeln!(
                log,
                "*** PANIC *** 'main' panicked at '{}', {}:{}:{}\n",
                msg, file, line, column
            ).unwrap();
        }
    }

    ::core::intrinsics::abort()
}

#[cfg(feature = "serial_panic")]
mod serial_panic {
    use core::fmt::{Error, Write};

    use uart;

    /// Setup a serial TX where the panic will be sent
    pub unsafe fn panic_on_serial(uart: uart::Uarts) {
        let uart = uart::Uart::setup(
            uart,
            57_600,
            uart::NBits::_8bits,
            uart::StopBits::_1b,
            uart::Parity::None,
        );
        LOG = Some(SerialLog(uart));
    }

    pub(crate) static mut LOG: Option<SerialLog> = None;

    pub(crate) struct SerialLog(uart::Uart);
    impl SerialLog {
        fn log(&mut self, s: &str) {
            for &b in s.as_bytes() {
                while !self.0.transmit_complete() {}
                self.0.send(b);
            }
        }
    }
    impl Write for SerialLog {
        fn write_str(&mut self, s: &str) -> Result<(), Error> {
            self.log(s);
            Ok(())
        }
    }
    pub unsafe fn trace(s: &str) {
        if let Some(ref mut log) = LOG {
            log.log("*** TRACE *** '");
            log.log(s);
            log.log("'\n");
        }
    }
}
#[cfg(feature = "serial_panic")]
pub use self::serial_panic::{panic_on_serial, trace};
