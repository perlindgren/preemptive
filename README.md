# Preemptive Regions inside Critical Sections

The [critical-section](https://github.com/rust-embedded/critical-section) crate provides closure based nested critical sections, with the guarantee that:

- The state is restored (if enabled and implemented for the target). This allows for correct by construction state handling if case of nesting.
- The critical section closure is passed a zero sized un-forgeable and un-leakable token, which can be used as proof for accessing shared resources.

While these properties are well motivated the implementation comes with two major shortcomings.

1. There is no defined way to provide regions of code allowing for preemption while inside a critical section.
2. The `CriticalSection` (`CS`) token implements `Copy` and `Clone`, which would allow the `CS` token to leak into inner preemptive regions.

This crate provides a workaround to 1., and the companying fork [critical-section](https://github.com/rust-embedded/critical-section) fixes 2.

This should be seen as a proof of concept and not a solution to the underlying problem. The unstructured `RawRestoreState` and its Rust feature based definition seems arbitrary chosen as motivation is lacking. Further investigation of the need for metadata (besides the binary state) is needed.
