#![no_main]
#![no_std]

use fugit::{Rate, RateExtU32};
use stm32f4xx_hal::{gpio::alt::tim1, pac, prelude::*, timer::{Channel, Timer}};
use cortex_m_rt::entry;
use panic_halt as _;
// use defmt_rtt as _;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut delay = dp.TIM1.delay_ms(&clocks);

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    let mut led_pin = gpioa.pa5.into_alternate();
    let button = gpioc.pc13.into_pull_down_input();

    let timer = Timer::new(dp.TIM2, &clocks);

    let rate = Rate::<u32, 1_000, 1>::from_raw(1);

    let tim2 = dp.TIM2;

    let tim1 = gpioa.pa0.into_alternate();

    let dp_tim1 = dp.TIM1;

    let frequency = 50.Hz();

    let pwm = dp_tim1.pwm_us(100.micros(), &clocks);

    let mut pwm_disabled = pwm.1;

    let mut pwm_c1 = pwm_disabled.0.with(tim1);

    let max_duty = pwm.0.get_max_duty();

    pwm_c1.set_duty(0);
    
    pwm_c1.enable();

    let mut brightness_level: u8 = 0;

    loop {
        if button.is_low() {
            delay.delay_ms(50_u32);

            if button.is_low() {
                brightness_level = (brightness_level + 1) % 6;

                let duty = (brightness_level as u16 * max_duty) / 5;
                pwm_c1.set_duty(duty);

                while button.is_low() {
                    // do nothing
                }
            }
        }
    }
}