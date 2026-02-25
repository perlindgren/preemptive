use core::cell::Cell;
use critical_section::Mutex;

// Old Mutex API, taking ownership of the CS token
static MY_VALUE: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

use critical_section as _;

fn main() {
    println!("std_cs");
    critical_section::with(|cs| {
        // This code runs within a critical section.
        println!("code is running in a critical section");

        // `cs` is a token that you can use to "prove" that to some API,
        // for example to a `Mutex`:
        println!("val {}", MY_VALUE.borrow(cs).get());

        // Using the old API this does not compile, because `cs` is moved into the closure of `borrow`
        // println!("val {}", MY_VALUE.borrow(cs).get());
    });
}
