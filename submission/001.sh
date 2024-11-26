# What is the hash of block 654,321?
#!/bin/bash
cd rust_programs
cargo build --release
cd target/release
./rust_programs 001
