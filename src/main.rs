#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux::init();

    let ms = 50_u8;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(ms);
            leds[curr].off().ok();
            delay.delay_ms(ms);
        }
    }
}
