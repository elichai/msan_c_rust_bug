# run.sh Output
```
==262830==WARNING: MemorySanitizer: use-of-uninitialized-value
    #0 0x56224a5b7299 in parse_data /tmp/msan_c_rust_bug/src/parsing.c:3:9
    #1 0x56224a5b6e79 in msan_c_rust_bug::main::h276a746f6d711a7c /tmp/msan_c_rust_bug/src/main.rs:13:9
    #2 0x56224a5b64b3 in core::ops::function::FnOnce::call_once::h14b11d8cffab8177 /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
    #3 0x56224a5b6fd6 in std::sys_common::backtrace::__rust_begin_short_backtrace::hdf9c13ea173c9cea /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:121:18
    #4 0x56224a5b62fc in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h62fe82ee8aa36441 /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    #5 0x56224a789ffc in core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h48bd4e21e728df6e /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:287:13
    #6 0x56224a874237 in std::panicking::try::do_call::hab21dc63b6cce441 /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #7 0x56224a879abb in __rust_try std.07db31f5-cgu.5
    #8 0x56224a8719ae in std::panicking::try::h23967870f273bc6a /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #9 0x56224a5d6ed1 in std::panic::catch_unwind::h46b9ef396187140e /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:140:14
    #10 0x56224a86ba5d in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h3ea64a12446b5832 /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    #11 0x56224a873e69 in std::panicking::try::do_call::h5ea217910b1cbc1b /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #12 0x56224a879abb in __rust_try std.07db31f5-cgu.5
    #13 0x56224a872eae in std::panicking::try::hc7d8ada8a17bec37 /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #14 0x56224a5d7011 in std::panic::catch_unwind::hdc0709f52b6f4d66 /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:140:14
    #15 0x56224a86b25e in std::rt::lang_start_internal::h2db2cc202cc997cd /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    #16 0x56224a5b6189 in std::rt::lang_start::hc265eee0de597901 /home/elichai2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    #17 0x56224a5b6f2e in main (/tmp/msan_c_rust_bug/target/x86_64-unknown-linux-gnu/debug/msan_c_rust_bug+0x90f2e) (BuildId: 456f5c1a88836ceaea7ab6959d56bc7c2266688b)
    #18 0x7f379aadc28f  (/usr/lib/libc.so.6+0x2328f) (BuildId: 768945cdf5e5796c2ab39f38ed160748fd94d12e)
    #19 0x7f379aadc349 in __libc_start_main (/usr/lib/libc.so.6+0x23349) (BuildId: 768945cdf5e5796c2ab39f38ed160748fd94d12e)
    #20 0x56224a550214 in _start (/tmp/msan_c_rust_bug/target/x86_64-unknown-linux-gnu/debug/msan_c_rust_bug+0x2a214) (BuildId: 456f5c1a88836ceaea7ab6959d56bc7c2266688b)

SUMMARY: MemorySanitizer: use-of-uninitialized-value /tmp/msan_c_rust_bug/src/parsing.c:3:9 in parse_data
  ORIGIN: invalid (0). Might be a bug in MemorySanitizer origin tracking.
    This could still be a bug in your code, too!
Exiting
```

## Rust version
```
rustc 1.68.0-nightly (52372f9c7 2023-01-21)
binary: rustc
commit-hash: 52372f9c71d8ade4cb815524f179119656f0aa2e
commit-date: 2023-01-21
host: x86_64-unknown-linux-gnu
release: 1.68.0-nightly
LLVM version: 15.0.6
```

## Clang version
```
clang version 15.0.7 (https://github.com/llvm/llvm-project.git 8dfdcc7b7bf66834a761bd8de445840ef68e4d1a)
Target: x86_64-unknown-linux-gnu
Thread model: posix
InstalledDir: /usr/local/bin
```