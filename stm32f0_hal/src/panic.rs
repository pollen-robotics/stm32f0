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
        if let Some(ref mut tx) = serial_panic::TX {
            writeln!(
                tx,
                "*** PANIC *** 'main' panicked at '{}', {}:{}:{}",
                msg, file, line, column
            ).unwrap();
        }
    }

    ::core::intrinsics::abort()
}

#[cfg(feature = "serial_panic")]
mod serial_panic {
    use core::fmt::{Error, Write};

    use hal;

    /// Setup a serial TX where the panic will be sent
    pub unsafe fn log_on_serial<W>(tx: &'static mut W)
    where
        W: hal::serial::Write<u8, Error = !>,
    {
        TX = Some(SerialLog(tx));
    }

    pub(crate) static mut TX: Option<SerialLog> = None;

    pub(crate) struct SerialLog(&'static mut hal::serial::Write<u8, Error = !>);
    impl Write for SerialLog {
        fn write_str(&mut self, s: &str) -> Result<(), Error> {
            for &b in s.as_bytes() {
                block!(self.0.write(b)).ok();
            }

            Ok(())
        }
    }
}
#[cfg(feature = "serial_panic")]
pub use self::serial_panic::log_on_serial;
