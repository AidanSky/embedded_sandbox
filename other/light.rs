#![no_main]
#![no_std]

use stm32f4xx_hal::{pac, prelude::*};
use cortex_m_rt::entry;
use panic_halt as _;
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        led.toggle();
        cortex_m::asm::delay(16_000_000);
    }
}