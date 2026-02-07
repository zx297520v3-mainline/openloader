cargo +nightly b --release -Z build-std=core,alloc
llvm-objcopy -O binary target/thumbv6m-none-eabi/release/openloader target/thumbv6m-none-eabi/release/openloader
