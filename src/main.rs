#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;

use microbit::led;
// // extern crate panic_halt;
// // use panic_halt as _;

// use microbit::display;
// // use cortex_m_rt::entry;
// use microbit::hal::prelude::*;
// use microbit::display::image::GreyscaleImage;
// use microbit::display::{self, Display, Frame, MicrobitDisplayTimer, MicrobitFrame};
// use microbit::hal::lo_res_timer::{LoResTimer,FREQ_512HZ,FREQ_32768HZ};
// use microbit::hal::nrf51;
// use rtfm::app;

// use cortex_m_rt::entry;

// // use microbit::hal::delay::Delay;
// // use microbit::hal::prelude::*;

// use microbit::led;
mod bitmaps;


#[entry]
fn main() -> ! {

     if let Some(p) = microbit::Peripherals::take() {
        let gpio = p.GPIO.split();
        let mut delay = Delay::new(p.TIMER0);

        let mut  display = microbit::led::Display::new(
            gpio.pin4.into_push_pull_output(),
            gpio.pin5.into_push_pull_output(),
            gpio.pin6.into_push_pull_output(),
            gpio.pin7.into_push_pull_output(),
            gpio.pin8.into_push_pull_output(),
            gpio.pin9.into_push_pull_output(),
            gpio.pin10.into_push_pull_output(),
            gpio.pin11.into_push_pull_output(),
            gpio.pin12.into_push_pull_output(),
            gpio.pin13.into_push_pull_output(),
            gpio.pin14.into_push_pull_output(),
            gpio.pin15.into_push_pull_output());

        // Configure button GPIOs as inputs
        let button_a = gpio.pin17.into_floating_input();
        let button_b = gpio.pin26.into_floating_input();
        display.display_pre(&mut delay, bitmaps::img::image_to_preformated_vector(bitmaps::img::square_image) , 300);
        display.display_pre(&mut delay, bitmaps::img::image_to_preformated_vector(bitmaps::img::square_small_image), 300);
        display.display_pre(&mut delay, bitmaps::img::image_to_preformated_vector(bitmaps::img::dot33) , 300);
        display.display_pre(&mut delay, bitmaps::img::image_to_preformated_vector(bitmaps::img::diamond_small_image) , 300);
        display.display_pre(&mut delay, bitmaps::img::image_to_preformated_vector(bitmaps::img::diamond_image) , 300);
        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];
        let image3 = bitmaps::img::image_to_preformated_vector(0x05db0975);
        let image0 = bitmaps::img::image_to_preformated_vector(bitmaps::img::yes_image);
        let image1 = bitmaps::img::image_to_preformated_vector(bitmaps::img::no_image);
        let image2 = bitmaps::img::image_to_preformated_vector(bitmaps::img::dot33);
        let image4 = microbit::led::Display::display2matrix(letter_I);


        loop {
            match (button_a.is_high(),button_b.is_high()) {
                (Ok(true),Ok(true)) => {
                    display.display_pre(&mut delay, image0,300);
                },
                (Ok(false),Ok(false))=> {
                    display.display_pre(&mut delay, image1,300);
                },
                (Ok(true),Ok(false))=> {
                    display.display_pre(&mut delay, image2,300);
                },
                (Ok(false),Ok(true))=> {
                    display.display_pre(&mut delay, image3,300);
                }
                _ => {
                    // Error case - at least one GPIO returned an error
                    display.display_pre(&mut delay, image4,800);
                }
                
            }
        }
    }

    loop {
        continue;
    }
}

// #[test]
// fn test_image_to_preformated_vector() {
//     let ans = super::image_to_preformated_vector(0x05db0975);


// }

