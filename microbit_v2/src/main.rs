#![no_main]
#![no_std]

use microbit::display::blocking::Display;
use microbit::{hal, Board};
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use microbit_bitmaps as bitmaps;

#[entry]
fn main() -> ! {
    // take the board
    if let Some(board) = Board::take() {
        let mut delay = hal::Timer::new(board.TIMER0);

        let mut display = Display::new(board.display_pins);

        // Configure button GPIOs as inputs
        let mut button_a = board.buttons.button_a;
        let mut button_b = board.buttons.button_b;
        display.show(
            &mut delay,
            bitmaps::img::image_to_5x5(bitmaps::img::square_image),
            300,
        );
        display.show(
            &mut delay,
            bitmaps::img::image_to_5x5(bitmaps::img::square_small_image),
            300,
        );
        display.show(
            &mut delay,
            bitmaps::img::image_to_5x5(bitmaps::img::dot33),
            300,
        );
        display.show(
            &mut delay,
            bitmaps::img::image_to_5x5(bitmaps::img::diamond_small_image),
            300,
        );
        display.show(
            &mut delay,
            bitmaps::img::image_to_5x5(bitmaps::img::diamond_image),
            300,
        );
        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];
        let image3 = bitmaps::img::image_to_5x5(0x05db0975);
        let image0 = bitmaps::img::image_to_5x5(bitmaps::img::yes_image);
        let image1 = bitmaps::img::image_to_5x5(bitmaps::img::no_image);
        let image2 = bitmaps::img::image_to_5x5(bitmaps::img::dot33);
        let image4 = letter_I;

        loop {
            match (button_a.is_high(), button_b.is_high()) {
                (Ok(true), Ok(true)) => {
                    display.show(&mut delay, image0, 300);
                }
                (Ok(false), Ok(false)) => {
                    display.show(&mut delay, image1, 300);
                }
                (Ok(true), Ok(false)) => {
                    display.show(&mut delay, image2, 300);
                }
                (Ok(false), Ok(true)) => {
                    display.show(&mut delay, image3, 300);
                }
                _ => {
                    // Error case - at least one GPIO returned an error
                    display.show(&mut delay, image4, 800);
                }
            }
        }
    }

    loop {
        continue;
    }
}
