#!/bin/bash

# Check if Cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found. Please install Rust and Cargo before running this script."
    exit 1
fi

# Change to the project directory
cd "$(dirname "$0")/distrohoop"

# Compile the project
cargo build --release

# Move the binary to /usr/local/bin
sudo mv target/release/distrohoop /usr/local/bin/

# Make sure the binary is executable
sudo chmod +x /usr/local/bin/distrohoop

echo "distrohoop has been installed. You can now run it by typing 'distrohoop' in your terminal =)"