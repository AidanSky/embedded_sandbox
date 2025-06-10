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
    let gpioc = dp.GPIOC.split();

    let mut led = gpioa.pa5.into_push_pull_output();
    let button = gpioc.pc13.into_pull_down_input();

    let mut _counter: u8 = 0;

    loop {
        if button.is_high() {
            led.set_low();
        } else {
            led.set_high();
        }
    }
}