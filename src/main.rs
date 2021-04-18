#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );

    let mut trig_pin = pins.d7.into_output(&mut pins.ddr);
    let echo_pin = pins.d6.into_pull_up_input(&mut pins.ddr);

    let white1 = pins.d8.into_output(&mut pins.ddr);
    let white2 = pins.d9.into_output(&mut pins.ddr);
    let yellow1 = pins.d10.into_output(&mut pins.ddr);
    let yellow2 = pins.d11.into_output(&mut pins.ddr);
    let red1 = pins.d12.into_output(&mut pins.ddr);
    let red2 = pins.d13.into_output(&mut pins.ddr);
    let buzzer = pins.d3.into_output(&mut pins.ddr);

    ufmt::uwriteln!(&mut serial, "Hello from Distance Detector!\r").void_unwrap();
    loop {
        trig_pin.set_low().void_unwrap();
        arduino_uno::delay_us(5);
        trig_pin.set_high().void_unwrap();
        arduino_uno::delay_us(10);
        trig_pin.set_low().void_unwrap();
        // Need to figure out how to do pulseIn
    }
}
