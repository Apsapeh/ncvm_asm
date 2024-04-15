cargo +nightly bloat -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort\
     --target aarch64-apple-darwin --debug -n 10