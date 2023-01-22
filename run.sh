#!/usr/bin/env bash
cargo +nightly clean
export CC='clang -fsanitize=memory -fno-omit-frame-pointer'
export RUSTFLAGS='-Zsanitizer=memory -Zsanitizer-memory-track-origins -Cforce-frame-pointers=yes'
cargo +nightly run -Zbuild-std --target x86_64-unknown-linux-gnu
