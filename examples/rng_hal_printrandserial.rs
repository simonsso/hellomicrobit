#![no_main]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate microbit;
extern crate panic_halt;

use microbit::cortex_m;
use microbit::hal::prelude::*;
use microbit::hal::rng;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

extern crate rand;
use rand::SeedableRng;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

static RTC: Mutex<RefCell<Option<microbit::RTC0>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<serial::Tx<microbit::UART0>>>> = Mutex::new(RefCell::new(None));
static RNG: Mutex<RefCell<Option<rand::ChaChaRng>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {
            p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

            while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}

            /* And then set it back to 0 again, just because ?!? */
            p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

            /* Split GPIO pins */
            let gpio = p.GPIO.split();

            /* Configure RX and TX pins accordingly */
            let tx = gpio.pin24.into_push_pull_output().downgrade();
            let rx = gpio.pin25.into_floating_input().downgrade();

            /* Set up serial port using the prepared pins */
            let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

            let _ = write!(tx, "\n\rWelcome to the random number printer!\n\r");

            /* Use hardware RNG to initialise PRNG */
            let mut rng = rng::Rng::new(p.RNG);

            let mut seed: [u8; 32] = [0; 32];

            /* Read 4 bytes of data from hardware RNG */
            rng.read(&mut seed).ok();

            let rng = rand::ChaChaRng::from_seed(seed);
            *RNG.borrow(cs).borrow_mut() = Some(rng);

            p.RTC0.prescaler.write(|w| unsafe { w.bits(1) });
            p.RTC0.evtenset.write(|w| w.tick().set_bit());
            p.RTC0.intenset.write(|w| w.tick().set_bit());
            p.RTC0.tasks_start.write(|w| unsafe { w.bits(1) });

            *RTC.borrow(cs).borrow_mut() = Some(p.RTC0);
            *TX.borrow(cs).borrow_mut() = Some(tx);

            if let Some(mut p) = Peripherals::take() {
                p.NVIC.enable(microbit::Interrupt::RTC0);
                microbit::NVIC::unpend(microbit::Interrupt::RTC0);
            }
        });
    }

    loop {
        continue;
    }
}

/* Define an exception, i.e. function to call when exception occurs. Here if our SysTick timer
 * trips the hello_world function will be called */
interrupt!(RTC0, printrng);
fn printrng() {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let (Some(rtc), &mut Some(ref mut rng), &mut Some(ref mut tx)) = (
            RTC.borrow(cs).borrow().as_ref(),
            RNG.borrow(cs).borrow_mut().deref_mut(),
            TX.borrow(cs).borrow_mut().deref_mut(),
        ) {
            use rand::Rng;
            let _ = write!(tx, "{}\n\r", rng.gen::<u32>());
            rtc.events_tick.write(|w| unsafe { w.bits(0) });
        }
    });
}
