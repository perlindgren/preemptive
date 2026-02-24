use critical_section::{CriticalSection, Impl, RawRestoreState, RestoreState};

struct MyCriticalSection;

impl MyCriticalSection {
    fn preemption<R>(cs: CriticalSection, f: impl FnOnce() -> R) -> (R, CriticalSection) {
        unsafe { critical_section::release(RestoreState::invalid()) }; // enable interrupts, invalid == false

        let result = f();

        unsafe { critical_section::acquire() }; // disable interrupts 
        (result, cs)
    }
}

critical_section::set_impl!(MyCriticalSection);

unsafe impl Impl for MyCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        println!("disable interrupts");
        false // for this example our return state should enable interrupts 
    }

    unsafe fn release(restore_state: RawRestoreState) {
        println!(
            "release critical section with restore state: {}",
            restore_state
        );
        if restore_state {
            println!("disable interrupts");
        } else {
            println!("enable interrupts");
        }
    }
}

mod private {
    use super::*;
    pub struct MyProtectedData {
        value: i32,
    }

    impl MyProtectedData {
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        pub fn access(&self, cs: &CriticalSection) -> i32 {
            // Access the protected data within the critical section
            println!("Accessing protected data: {}", self.value);
            self.value
        }
    }
}

use private::MyProtectedData;
fn main() {
    let protected_data = MyProtectedData::new(42);

    critical_section::with(|cs| {
        // Critical section code goes here
        // You can use `cs` to manage the critical section state
        println!("Inside critical section");
        protected_data.access(&cs);

        // The CS token is unaccessible inside the closure
        let (_, cs) = MyCriticalSection::preemption(cs, || {
            println!("Running with preemption enabled");

            // As you are not is the CS this will fal
            // protected_data.access(&cs);
        });

        MyCriticalSection::preemption(cs, || {
            println!("Running with preemption enabled");
        });

        //cs // you cannot leak the critical section token
    });
}
