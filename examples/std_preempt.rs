use preemption::{Mutex, preemptive_region};
static MY_VALUE: Mutex<i32> = Mutex::new(0);

use critical_section as _;

fn main() {
    println!("std_preempt");
    critical_section::with(|mut cs| {
        println!("val {}", MY_VALUE.read(&cs, |data| *data));
        MY_VALUE.write(&mut cs, |data| *data = 42);
        println!("val {}", MY_VALUE.read(&cs, |data| *data));

        // Will error: cannot borrow `cs` as mutable more than once at a time
        // MY_VALUE.write(&mut cs, |data| {
        //     MY_VALUE.write(&mut cs, |data| *data);
        //     *data += 1;
        // });

        preemptive_region::with(&mut cs, || {
            println!("in preemptive region");
            // The CS token is unaccessible inside the closure
            // MY_VALUE.read(&cs, |data| *data); // <-- compile error: borrow of moved value: `cs`
        });

        critical_section::with(|mut cs| {
            println!("in nested critical section");
            println!("val {}", MY_VALUE.read(&cs, |data| *data));
            preemptive_region::with(&mut cs, || {
                println!("in cs nested preemptive region");
                // The CS token is unaccessible inside the closure
                //MY_VALUE.read(&cs, |data| *data); // <-- compile error: borrow of moved value: `cs`
            });
        });

        preemptive_region::with(&mut cs, || {
            println!("in preemptive region");
            // The CS token is unaccessible inside the closure
            // MY_VALUE.read(&cs, |data| *data); // <-- compile error: borrow of moved value: `cs`
        });

        MY_VALUE.write(&mut cs, |data| *data = 1337);
        println!("val {}", MY_VALUE.read(&cs, |data| *data));

        // cs // <-- compile error: lifetime may not live long enough, thus cannot be be leaked
    });
}
