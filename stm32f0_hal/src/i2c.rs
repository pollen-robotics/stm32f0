use embedded_hal::blocking::i2c;
use stm32f0x2::{I2C1, GPIOB, RCC};

/// Interface to the I2C bus
///
/// This is only a first iteration on the I2C interface. Currently it comes with
/// the following limitations:
/// - It only supports I2C1.
/// - It can only use PB6 for SCL and PB7 for SDA.
/// - It can only use the HSI as the clock source.
/// - The I2C frequency is hard-coded to 100kHz.
/// - Some other limitations are documented on the affected methods.
///
/// In addition, this API does all its work by accessing the registers directly,
/// which might interfere with other modules that use the same registers. Please
/// be careful when using this API together with other APIs, specifically
/// anything GPIO-related.
pub struct I2C<'r> {
    i2c: &'r I2C1,
}

impl<'r> I2C<'r> {
    /// Initializes the I2C1 peripheral
    pub fn init(rcc: &RCC, gpiob: &GPIOB, i2c: &'r I2C1) -> Self {
        // Enable clock for GPIOB
        rcc.ahbenr.modify(|_, w| w.iopben().enabled());

        // Enable clock for I2C1
        rcc.apb1enr.modify(|_, w| w.i2c1en().enabled());

        // Reset I2C1 peripheral
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().clear_bit());

        // Select alternate I2C functions on pins
        // See data sheet, page 42 and reference manual, section 8.4.9.
        gpiob.afrl.modify(
            |_, w| {
                w
                .afrl6().af1() // I2C1_SCL on PB6
                .afrl7().af1()
            }, // I2C1_SDA on PB7
        );

        // Configure the pins
        //
        // I'm basically winging it here. If the need for setting the
        // configuration like this is explained somewhere in the reference
        // manual, I don't know where. The following is based on my own
        // understanding, and bits and pieces I've gathered from around the
        // internet.
        //
        // Fact is, the pins need to be configured (there's no I2C signal
        // otherwise), and the following configuration works.
        gpiob
            .moder
            .modify(|_, w| w.moder6().alternate().moder7().alternate());
        gpiob
            .ospeedr
            .modify(|_, w| w.ospeedr6().low_speed().ospeedr7().low_speed());
        gpiob
            .otyper
            .modify(|_, w| w.ot6().open_drain().ot7().open_drain());

        // Select HSI as the clock source for I2C1
        // This is the default, so unless other code has changed this
        // configuration, the following line is redundant.
        rcc.cfgr3.modify(|_, w| w.i2c1sw().hsi());

        // Make sure the I2C peripheral is disabled
        // The configuration below is not allowed, while I2C is enabled.
        i2c.cr1.modify(|_, w| w.pe().clear_bit());

        // Noise filter configuration would go here, if required. By default, a
        // spec-compliant analog filter is enabled, and I don't see a reason to
        // change that.

        // Configure I2C timings
        // The following values are taken from table 91 on page 650 (section
        // 26.4.10) in the reference manual. This is the right table because the
        // 8 MHz HSI clock has been selected above. Let's choose 100 kHz for the
        // I2C frequency.
        i2c.timingr.modify(|_, w| unsafe {
            w
                    .presc().bits(1)
                    .scll().bits(0x13) // required because of master mode
                    .sclh().bits(0xf)  // required because of master mode
                    .sdadel().bits(0x2)
                    .scldel().bits(0x4)
        });

        // Enable I2C peripheral
        i2c.cr1.modify(|_, w| w.pe().set_bit());

        I2C { i2c: i2c }
    }
}

impl<'r> i2c::Write for I2C<'r> {
    type Error = WriteError;

    /// Write to the I2C bus
    ///
    /// Writes the data to the I2C bus. Blocks until all data has been written.
    ///
    /// This method has the following limitations:
    /// - It doesn't check for error conditions.
    /// - It only supports writing up to 255 bytes at a time.
    /// - Writing more than 1 byte should work, but hasn't been tested.
    fn write(&mut self, address: u8, data: &[u8]) -> Result<(), WriteError> {
        let nbytes = if data.len() <= u8::max_value() as usize {
            data.len() as u8
        } else {
            return Err(WriteError::DataTooLong);
        };

        // Configure and start I2C write
        self.i2c.cr2.write(|w| unsafe {
            w
                    // 7-bit addressing mode
                    .add10().clear_bit()
                    // slave address
                    .sadd1().bits(address >> 1)
                    .sadd0().bit(false) // write
                    // write config
                    .rd_wrn().clear_bit()  // set transfer direction to write
                    .nbytes().bits(nbytes) // set number of bytes to send
                    .reload().clear_bit()  // only send <nbytes> bytes
                    .autoend().set_bit()   // automatically send STOP signal
                    .start().set_bit() // start transmission
        });

        for &b in data {
            // Wait for transmit register to be empty
            while self.i2c.isr.read().txe().bit_is_clear() {}

            // Send index of reference register
            self.i2c.txdr.write(|w| w.txdata().bits(b));
        }

        // Wait until transfer is complete
        // Since we set the AUTOEND flag above, the peripheral should
        // automatically generate a stop condition.
        while self.i2c.isr.read().stopf().bit_is_clear() {}

        Ok(())
    }
}

impl<'r> i2c::Read for I2C<'r> {
    type Error = ReadError;

    /// Read from the I2C bus
    ///
    /// Reads data from the I2C bus until the buffer has been filled. Blocks
    /// until all bytes have been read.
    ///
    /// This method has the following limitations:
    /// - It doesn't check for error conditions.
    /// - It only supports reading up to 255 bytes at a time.
    /// - Reading more than 1 byte should work, but hasn't been tested.
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), ReadError> {
        let nbytes = if buffer.len() <= u8::max_value() as usize {
            buffer.len() as u8
        } else {
            return Err(ReadError::BufferTooLong);
        };

        // Configure and start I2C read
        self.i2c.cr2.write(|w| unsafe {
            w
                    // 7-bit addressing mode
                    .add10().clear_bit()
                    // slave address
                    .sadd1().bits(address >> 1)
                    .sadd0().bit(true) // read
                    // read config
                    .rd_wrn().set_bit()   // set transfer direction to read
                    .nbytes().bits(nbytes)     // receive 1 byte
                    .reload().clear_bit() // only receive <nbytes> bytes
                    .autoend().set_bit()  // automatically send STOP signal
                    .start().set_bit() // start signal to start transmission
        });

        for b in buffer {
            // Wait until byte has been received
            while self.i2c.isr.read().rxne().bit_is_clear() {}

            // Read received byte
            *b = self.i2c.rxdr.read().rxdata().bits();
        }

        Ok(())
    }
}

pub enum WriteError {
    DataTooLong,
}

pub enum ReadError {
    BufferTooLong,
}
