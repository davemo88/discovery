#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let step = 50_u8;

    loop {
        for i in 0..8 {
            let next = (i + 1) % 8;
            leds[next].on().ok();
            delay.delay_ms(step);
            leds[i].off().ok();
            delay.delay_ms(step);
        }
    }
}
