fn main() {
    /*
     * UNREVOCERABLE ERRORS WITH panic!
     *
     * UNWINDING THE STACK OR ABORTING IN RESPONSE TO A PANIC
     *
     * Unwindly means that Rust walks back up the stack and cleans up the from
     * each function it encounters. Buti this walking back and cleanup is a lot
     * of work.
     *
     * Alternatively Rust can abort, which ends the program without cleaning
     * up. Memory must be cleaned by the Operative System.
     *
     * If in your project you need to make the resulting binary as small as 
     * possible, you can switch from unwinding to aborting upon a panic by
     * adding panic = 'abort' to the appropiate [profile] sections in your
     * Cargo.toml.
     *
     * Example to abort on panic in release mode:
     *
     * [profile.release]
     * panic = 'abort'
     *
     */

    panic!("crash and burn");

    /*
     * USING A PANIC BACKTRACE
     *
     * panic! can be called from other libraries we used, for example when we
     * try to access to an inexistent index in a vector:
     *
     * let v = vec![1, 2, 3];
     *
     * v[99];
     *
     * Using [] Rust supposed to return an element, but there's no element at
     * 99th index.
     *
     * In C it has an undefined behaviour: wou might get whatever is at the 
     * location in memory that wold correspond to that element. This is called
     * buffer overread and ban lead to security vulnerbilities.
     *
     * The result will be:
     *
     * $ cargo run
     *   Compiling panic v0.1.0 (file:///projects/panic)
     *    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     *     Running `target/debug/panic`
     * thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
     * note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     *
     * Setting the environment variable RUST_BACKTRACE=1 we can follow the file
     * that originates the panic!:
     *
     * $ RUST_BACKTRACE=1 cargo run
     * thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
     * stack backtrace:
     * 0: rust_begin_unwind
     *        at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
     * 1: core::panicking::panic_fmt
     *        at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
     * 2: core::panicking::panic_bounds_check
     *        at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
     * 3: <usize as core::slice::index::SliceIndex<[T]>>::index
     *        at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
     * 4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
     *        at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
     * 5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
     *        at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
     * 6: panic::main
     *        at ./src/main.rs:4
     * 7: core::ops::function::FnOnce::call_once
     *        at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
     * note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
     *
     * The exact output you see might be different depending on your operating
     * system and Rust version.
     *
     * In order to get backtraces with this information, debug symbols must be
     * enabled. Debug symbols are enabled by default when using cargo build or
     * cargo run without the --release flag, as we have here.
     *
     */
}
