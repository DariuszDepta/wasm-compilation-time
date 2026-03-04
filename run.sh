cargo build --release

./target/release/compilation-time wasmer-cranelift-none
./target/release/compilation-time wasmer-cranelift-speed
./target/release/compilation-time wasmer-singlepass
./target/release/compilation-time wasmtime-cranelift-none
./target/release/compilation-time wasmtime-cranelift-speed
./target/release/compilation-time wasmtime-singlepass
