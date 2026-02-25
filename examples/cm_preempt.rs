#![no_std]
#![no_main]

use cortex_m as _;
use cortex_m_rt::entry;
use panic_halt as _;

use preemption::{Mutex, preemptive_region};
static MY_VALUE: Mutex<i32> = Mutex::new(0);

#[entry]
fn main() -> ! {
    critical_section::with(|mut cs| {
        MY_VALUE.read(&cs, |data| *data);
        MY_VALUE.write(&mut cs, |data| *data);

        // Will error: cannot borrow `cs` as mutable more than once at a time
        // MY_VALUE.write(&mut cs, |data| {
        //     MY_VALUE.write(&mut cs, |data| *data);
        //     *data += 1;
        // });

        preemptive_region::with(&mut cs, || {
            // The CS token is unaccessible inside the closure
            // MY_VALUE.read(&cs, |data| *data); // <-- compile error: borrow of moved value: `cs`
        });

        critical_section::with(|mut cs| {
            MY_VALUE.read(&cs, |data| *data); // <-- compile error: borrow of moved value: `cs`
            preemptive_region::with(&mut cs, || {
                // The CS token is unaccessible inside the closure
                //MY_VALUE.read(&cs, |data| *data); // <-- compile error: borrow of moved value: `cs`
            });
        });

        // cs // <-- compile error: lifetime may not live long enough, thus cannot be be leaked
    });

    loop {}
}
