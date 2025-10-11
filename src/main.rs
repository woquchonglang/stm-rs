//! Blinks an LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let gpioh = p.GPIOH.split(&mut rcc);
    let mut led = gpioh.ph10.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            led.set_high();
            rprintln!("light on!");
        }
        for _ in 0..10_000 {
            led.set_low();
            rprintln!("light off!");
        }
    }
}
