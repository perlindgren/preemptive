# Preemptive Regions inside Critical Sections

The [critical-section](https://github.com/rust-embedded/critical-section) crate provides closure based nested critical sections, with the guarantee that:

- The state is restored (if enabled and implemented for the target). This allows for correct by construction state handling of nesting critical sections.
- The critical section closure is passed a zero sized un-forgeable and un-leakable token, which can be used as proof for accessing shared resources.

While these properties are well motivated the implementation comes with two major shortcomings.

1. There is no defined way to provide regions of code allowing for preemption while inside a critical section.
2. The `CriticalSection` (`CS`) token implements `Copy` and `Clone`, which would allow the `CS` token to leak into inner preemptive regions.

This crate provides a workaround to 1., and the companying fork [critical-section](https://github.com/rust-embedded/critical-section) fixes 2.

This should be seen as a proof of concept and not a solution to the underlying problem. The unstructured `RawRestoreState` and its Rust feature based definition seems arbitrary chosen as motivation is lacking. Further investigation of the need for metadata (besides the binary state) is needed.

The `Impl` trait definition is tailored to the exact needs for the current critical section implementation. By a more generic getter and setter design the support for preemptive regions would be facilitated. The workaround piggy backs on the definition of invalid state, which while working is unsatisfying.

## Mutex Design

We propose a new Closure based Mutex API, taking the CS by reference instead of as owned value. The old consuming API relies on the CS being `Copy` + `Clone` and cause leaking in context of preemptive regions.

## Examples

The examples include:

- `cm_test`, testing of the stock `cortex_m` CS implementation. Backing `Impl` comes from stock `cortex-m`:

  ```shell
  cargo build --example cm_test --target thumbv7em-none-eabihf --no-default-features --features cm
  ```

- `cm_preempt`, testing of the new CS and Mutex implementation. Backing `Impl` comes from stock `cortex-m`:

  ```shell
  cargo build --example cm_preempt --target thumbv7em-none-eabihf --no-default-features --features cm
  ```

- `std_cs`, shows the new CS API with the stock Mutex API. Backing `Impl` comes from the stock `critical-section`.

  ```shell
  cargo run --example std_cs
  ```

- `std_preempt`, shows the new CS API with the new Mutex API. Backing `Impl` comes from the stock `critical-section`.

  ```shell
  cargo run --example std_cs
  ```
  