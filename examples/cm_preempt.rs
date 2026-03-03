#![no_std]
#![no_main]

use core::panic;

use cortex_m as _;
use cortex_m_rt::entry;
use panic_halt as _;

use preemption::Mutex;
static MY_VALUE: Mutex<i32> = Mutex::new(0);

#[entry]
fn main() -> ! {
    critical_section::with(|mut cs| {
        MY_VALUE.with_ref_mut(&mut cs, |data| {
            cortex_m::asm::nop();
            // Would error: cannot borrow `cs` as mutable more than once at a time
            // MY_VALUE.with_ref(&cs, |data| *data);
            *data += 1;
        });

        critical_section::preemption_within(&mut cs, || {
            cortex_m::asm::nop();
            // Would error: cannot borrow `cs` as immutable because it is also borrowed as mutable
            // MY_VALUE.read(&cs, |data| *data);
        });

        cortex_m::asm::bkpt();

        MY_VALUE.with_ref_mut(&mut cs, |data| *data += 2);

        let r = critical_section::with(|mut cs| {
            cortex_m::asm::nop();
            let r = MY_VALUE.with_ref(&cs, |data| *data);
            critical_section::preemption_within(&mut cs, || {
                cortex_m::asm::nop();
                // Would error: cannot borrow `cs` as immutable because it is also borrowed as mutable
                // MY_VALUE.with_ref(&cs, |data| *data);
            });
            cortex_m::asm::bkpt();
            r
        });

        MY_VALUE.with_ref_mut(&mut cs, |data| *data = r);
        cortex_m::asm::bkpt();
        // cs // Would error: lifetime may not live long enough, thus cannot be be leaked
    });

    loop {}
}
