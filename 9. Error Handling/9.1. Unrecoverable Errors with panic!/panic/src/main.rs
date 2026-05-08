fn main() {
    // panic!("crash and burn");
    
    //panic occured without mentioning in the code 
    let v = vec![1,2,3];

    v[99];
}


// RUST_BACKTRACE=1 cargo run
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
//      Running `target/debug/panic`
//
// thread 'main' (17886) panicked at src/main.rs:7:6:
// index out of bounds: the len is 3 but the index is 99
// stack backtrace:
//    0: __rustc::rust_begin_unwind
//              at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
//    1: core::panicking::panic_fmt
//              at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
//    2: core::panicking::panic_bounds_check
//              at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:271:5
//    3: <usize as core::slice::index::SliceIndex<[T]>>::index
//              at /home/darshan/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust
// /library/core/src/slice/index.rs:272:10
//    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
//              at /home/darshan/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust
// /library/core/src/slice/index.rs:19:15
//    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
//              at /home/darshan/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust
// /library/alloc/src/vec/mod.rs:3804:9
//    6: panic::main
//              at ./src/main.rs:7:6
//    7: core::ops::function::FnOnce::call_once
//              at /home/darshan/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust
// /library/core/src/ops/function.rs:250:5
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

