#![no_std]

use critical_section::{CriticalSection, Impl, RawRestoreState, RestoreState};

pub mod preemptive_region {
    use super::*;
    /// Executes a closure with preemption enabled, inside a critical section.
    ///
    /// # Safety
    ///
    /// By requiring the CriticalSection (CS) token, we ensure that `with` can only
    /// be called from within a critical section.
    ///
    /// The CS token will mutably borrowed by the with function, thus
    /// inaccessible within the closure `f`.
    ///
    /// Given the assumption that RestoreState::invalid() represents a states
    /// where preemption is enabled, the closure `f` will thus execute:
    ///
    /// - Within a critical section
    /// - With preemption enabled
    /// - Without access to the CriticalSection token.
    ///

    pub fn with<R>(cs: &mut CriticalSection, f: impl FnOnce() -> R) -> R {
        unsafe { critical_section::release(RestoreState::invalid()) };

        let result = f();

        unsafe { critical_section::acquire() };
        result
    }

    /// Create a well-defined preemption point within a critical section.
    ///
    /// # Safety
    ///
    /// See `with` for safety properties.
    ///
    pub fn point(cs: &mut CriticalSection) {
        with(cs, || {})
    }
}

#[cfg(custom_cs)]
pub mod custom_cs {

    pub struct PreemptiveRegion;
    critical_section::set_impl!(PreemptiveRegion);

    unsafe impl Impl for PreemptiveRegion {
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
}

#[cfg(custom_cs)]
pub use custom_cs::*;

pub struct Mutex<T> {
    data: core::cell::UnsafeCell<T>,
}

// This is not actually safe, but serves only as an illustration
impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: core::cell::UnsafeCell::new(data),
        }
    }

    pub fn read<R>(&self, _cs: &CriticalSection, f: impl FnOnce(&T) -> R) -> R {
        // Access the protected data within the critical section
        let data = unsafe { &*self.data.get() };
        f(data)
    }

    pub fn write<R>(&self, _cs: &mut CriticalSection, f: impl FnOnce(&mut T) -> R) -> R {
        // Access the protected data within the critical section
        let data = unsafe { &mut *self.data.get() };
        f(data)
    }
}

unsafe impl<T> Sync for Mutex<T> {}
