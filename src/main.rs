#![no_main]
#![no_std]

extern crate panic_halt;
use panic_halt as _;

use cortex_m_rt::entry;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    // unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 2048 as usize) };

     if let Some(p) = microbit::Peripherals::take() {
    //     /* Split GPIO pins */
         let mut gpio = p.GPIO.split();
    //     let mut delay = hal::delay::Delay::new(p.TIMER0);
    //     let pxx = cortex_m::Peripherals::take().unwrap();

        // Set row of LED matrix to permanent high
        let _ = gpio.pin13.into_push_pull_output().set_high();

        // Set 2 columns to output to control LED states
        let mut led1 = gpio.pin4.into_push_pull_output();
        let mut led2 = gpio.pin6.into_push_pull_output();

        // Configure button GPIOs as inputs
        let button_a = gpio.pin17.into_floating_input();
        let button_b = gpio.pin26.into_floating_input();

        loop {
            if let Ok(true) = button_a.is_high() {
                let _ = led1.set_high();
            } else {
                let _ = led1.set_low();
            }

            if let Ok(true) = button_b.is_high() {
                let _ = led2.set_high();
            } else {
                let _ = led2.set_low();
            }
        }
    }

    loop {
        continue;
    }
}
