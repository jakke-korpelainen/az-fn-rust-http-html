#!/bin/bash

# Build for Windows
target="x86_64-pc-windows-msvc"
echo "Building for target: $target"
cargo build --release --target $target
if [ $? -ne 0 ]; then
    echo "Build failed for target: $target"
    # Wait for a signal to proceed
    echo "Waiting for signal to proceed..."
    read -p "Press [Enter] key to continue..."
fi
cp target/$target/release/handler.exe ./handler.exe

echo "Built and binaries copied successfully."