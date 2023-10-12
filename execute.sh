#!/bin/sh

cargo build --release

if [ $? -eq 0 ]; then
    ./target/release/benches "$@"
else
    echo "Build failed!"
fi
