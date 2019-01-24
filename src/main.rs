#![no_main]
#![no_std]
#![feature(alloc)]
#![feature(lang_items)]

extern crate cortex_m_rt;
extern crate alloc;
// #[cfg(not(test))]
extern crate panic_halt;
use core::alloc::Layout;

extern crate alloc_cortex_m;
extern crate cortex_m_rt as rt; // v0.5.x

#[macro_use(block)]
extern crate hellomicrobit as microbit;

extern crate cortex_m;

use cortex_m::asm;
use alloc_cortex_m::CortexMHeap;
use microbit::hal::prelude::*;
use microbit::hal::serial::BAUD115200;
use microbit::hal::serial;

extern crate embedded_hal;
use microbit::hal;
use hal::spi::SpiExt;


use cortex_m_rt::entry;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// the eink library
extern crate epd_waveshare;
use epd_waveshare::{
    epd1in54::{Buffer1in54, EPD1in54},
    graphics::{Display, DisplayRotation},
    prelude::*,
};
// Graphics
extern crate embedded_graphics;
use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
//use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics::Drawing;
pub mod bitmaps;
// use microbit::microbit_bitmaps as bitmaps;
#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 2048 as usize) };

    if let Some(p) = microbit::Peripherals::take() {
        /* Split GPIO pins */
        let mut gpio = p.GPIO.split();
        let mut delay = hal::delay::Delay::new(p.TIMER0);
        let pxx = cortex_m::Peripherals::take().unwrap();

        let syst = pxx.SYST;
        let clocks = p.CLOCK;
        /* Set row of // LED matrix to permanent high */

        let mut display = microbit::led::Display::new(
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

        display.display_pre_u32(&mut delay, bitmaps::img::square_image , 300);
        display.display_pre_u32(&mut delay, bitmaps::img::square_small_image, 300);
        display.display_pre_u32(&mut delay, bitmaps::img::dot33 , 300);
        display.display_pre_u32(&mut delay, bitmaps::img::diamond_small_image , 300);
        display.display_pre_u32(&mut delay, bitmaps::img::diamond_image , 300);

        /* Initialise serial port on the micro:bit */

        /* Configure RX and TX pins accordingly */
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();

        /* Set up serial port using the prepared pins */
        let (mut tx, mut rx) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

        let mut spi = p.SPI0.constrain( hal::spi::Pins{
                sck: gpio.pin23.into_push_pull_output().downgrade(),
                mosi: gpio.pin21.into_push_pull_output().downgrade(),
                miso: gpio.pin22.into_floating_input().downgrade()});

        let s = b"Hello connect BM Lite and start\r\n";
        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();


         // Conect pins for reset and IRQ  
        let mut cs = gpio.pin1.into_push_pull_output();    // Pad 2
        let mut rst = gpio.pin2.into_push_pull_output();   // Pad 1
        let mut busy = gpio.pin3.into_pull_up_input();      // Pad 0   // Setup the epd

    	let mut dc = gpio.pin20.into_push_pull_output();

        let mut  btn_a = gpio.pin17.into_pull_up_input();
        let mut  btn_b = gpio.pin26.into_pull_up_input();
        let mut epd = EPD1in54::new(&mut spi, cs, busy, dc, rst, &mut delay).unwrap();

        // Setup the graphics
        let mut buffer = Buffer1in54::default();
        let mut eink = Display::new(epd.width(), epd.height(), &mut buffer.buffer);

        // Draw some text
        eink.draw(
            Font6x8::render_str("シモンソン  Hello!   奈穂  åratal ")
                .with_stroke(Some(Color::Black))
                .with_fill(Some(Color::White))
                .translate(Coord::new(5, 50))
                .into_iter(),
        );

        // Transfer the frame data to the epd
        let _ans = epd.update_frame(&mut spi, &eink.buffer());

        // Display the frame on the epd
        let _ans2 = epd.display_frame(&mut spi);

        loop {
         
            display.display_pre_u32(&mut delay,bitmaps::img::minus,30);
            if btn_a.is_low(){

                eink.draw(
                Font6x8::render_str("Some more test ")
                        .with_stroke(Some(Color::Black))
                        .with_fill(Some(Color::White))
                        .translate(Coord::new(10, 50))
                        .into_iter(),
                );

                // Transfer the frame data to the epd
                let _ans = epd.update_frame(&mut spi, &eink.buffer());

                // Display the frame on the epd
                let _ans2 = epd.display_frame(&mut spi);

                display.display_pre_u32(&mut delay,bitmaps::img::question_mark,5000);
            }
            if btn_b.is_low(){
                display.display_pre_u32(&mut delay,bitmaps::img::hbars_top_botom,1000);
                eink.draw(
                    Font6x8::render_str("Another")
                        .with_stroke(Some(Color::White))
                        .with_fill(Some(Color::Black))
                        .translate(Coord::new(5, 25))
                        .into_iter(),
                );

                // Transfer the frame data to the epd
                let _ans = epd.update_frame(&mut spi, &eink.buffer());

                // Display the frame on the epd
                let _ans2 = epd.display_frame(&mut spi);
            }
        }
    }else{
        loop{

        }
    }
}


// required: define how Out Of Memory (OOM) conditions should be handled
// *if* no other crate has already defined `oom`
#[lang = "oom"]
#[no_mangle]

pub fn rust_oom(_layout: Layout) -> ! {
   // trap here for the debuger to find
   asm::bkpt();
   loop {
   }
}
