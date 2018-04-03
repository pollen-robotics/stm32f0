#![no_std]


extern crate cortex_m;
extern crate stm32f0x2;
extern crate stm32f0_hal;


use stm32f0x2::{
    GPIOB,
    I2C1,
    RCC,
};
use stm32f0_hal::gpio;


fn main() {
    // This example was developed and tested with the STM32F072B Discovery board
    // and the VL53L0X sattelite board. It assumes that the VL53L0X satellite
    // board is connected to the discovery board in the following way:
    // - GND to GND, VDD to 3V.
    // - XSDN_I to VDD, via a 10kOhm pull-up resistor.
    // - SCL_I to PB6, SDA_I to PB7.

    cortex_m::interrupt::free(|cs| {
        let rcc   = RCC.borrow(cs);
        let gpiob = GPIOB.borrow(cs);
        let i2c   = I2C1.borrow(cs);

        // Set up LEDs that we can later use to signal error or success
        //
        // These LEDs are marked as LD3, LD4, and LD5 on the STM32F0 Discovery
        // board. Please note that the board's documentation that matches those
        // names to PC6, PC8, and PC9 is not correct. It also mixes up the
        // colors of the LEDs. At least that's the case with the board I have in
        // front of me.
        let mut led_init    = gpio::Output::setup(gpio::Pin::PC8); // orange
        let mut led_success = gpio::Output::setup(gpio::Pin::PC9); // green
        let mut led_error   = gpio::Output::setup(gpio::Pin::PC6); // red

        led_init.high();
        led_success.low();
        led_error.low();

        // Enable clock for GPIOB
        //
        // I think this is already included in the GPIO output configuration
        // above, but it's more robust to do it again here explicitely, in case
        // that code changes, or this example switches to another method of
        // signalling success or failure.
        rcc.ahbenr.modify(|_, w| w.iopben().enabled());

        // Enable clock for I2C1
        rcc.apb1enr.modify(|_, w| w.i2c1en().enabled());

        // Reset I2C1 peripheral
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().clear_bit());

        // Select alternate I2C functions on pins
        // See data sheet, page 42 and reference manual, section 8.4.9.
        gpiob.afrl.modify(|_, w|
            w
                .afrl6().af1() // I2C1_SCL on PB6
                .afrl7().af1() // I2C1_SDA on PB7
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
        gpiob.moder.modify(|_, w|
            w
                .moder6().alternate()
                .moder7().alternate()
        );
        gpiob.ospeedr.modify(|_, w|
            w
                .ospeedr6().low_speed()
                .ospeedr7().low_speed()
        );
        gpiob.otyper.modify(|_, w|
            w
                .ot6().open_drain()
                .ot7().open_drain()
        );

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
        i2c.timingr.modify(|_, w|
            unsafe {
                w
                    .presc().bits(1)
                    .scll().bits(0x13) // required because of master mode
                    .sclh().bits(0xf)  // required because of master mode
                    .sdadel().bits(0x2)
                    .scldel().bits(0x4)
            }
        );

        // Enable I2C peripheral
        i2c.cr1.modify(|_, w| w.pe().set_bit());

        // Configure and start I2C write
        i2c.cr2.write(|w|
            unsafe {
                w
                    // 7-bit addressing mode
                    .add10().clear_bit()
                    // slave address
                    .sadd1().bits(0x52 >> 1)
                    .sadd0().bit(false) // write
                    // write config
                    .rd_wrn().clear_bit() // set transfer direction to write
                    .nbytes().bits(1)     // send 1 byte, the register index
                    .reload().clear_bit() // only send <nbytes> bytes
                    .autoend().set_bit()  // automatically send STOP signal
                    .start().set_bit()    // start signal to start transmission
            }
        );

        // Wait for transmit register to be empty
        while i2c.isr.read().txe().bit_is_clear() {}

        // Send index of reference register
        i2c.txdr.write(|w| w.txdata().bits(0xC0));

        // Wait until transfer is complete
        // Since we set the AUTOEND flag above, the peripheral should
        // automatically generate a stop condition.
        while i2c.isr.read().stopf().bit_is_clear() {}

        // Configure and start I2C read
        i2c.cr2.write(|w|
            unsafe {
                w
                    // 7-bit addressing mode
                    .add10().clear_bit()
                    // slave address
                    .sadd1().bits(0x52 >> 1)
                    .sadd0().bit(true) // read
                    // read config
                    .rd_wrn().set_bit()   // set transfer direction to read
                    .nbytes().bits(1)     // receive 1 byte
                    .reload().clear_bit() // only receive <nbytes> bytes
                    .autoend().set_bit()  // automatically send STOP signal
                    .start().set_bit()    // start signal to start transmission
            }
        );

        // Wait until byte has been received
        while i2c.isr.read().rxne().bit_is_clear() {}

        // Read received byte
        let byte = i2c.rxdr.read().rxdata().bits();

        // Check received data
        // We expect that it's the hardcoded value from the reference register
        // we were reading.
        if byte == 0xEE {
            led_success.high();
        }
        else {
            led_error.high();
        }
    });
}
