#![no_std]
#![no_main]

use core::cell::Cell;
use critical_section::Mutex;

// Old Mutex API, taking ownership of the CS token
static MY_VALUE: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

use cortex_m as _;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    critical_section::with(|cs| {
        // This code runs within a critical section.

        // `cs` is a token that you can use to "prove" that to some API,
        // for example to a `Mutex`:
        MY_VALUE.borrow(cs).set(42);
    });

    loop {}
}
