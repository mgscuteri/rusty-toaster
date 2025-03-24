#!/bin/bash

# Export the linker for aarch64-unknown-linux-gnu
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

# OpenSSL configuration
export OPENSSL_DIR=/usr
export OPENSSL_INCLUDE_DIR=/usr/include
export OPENSSL_LIB_DIR=/usr/lib

# Specify the custom SCP port
SCP_PORT=8907

# Define the Asustor user, IP/hostname, and target directories
ASUSTOR_USER="root"  # Updated to root for debugging purposes
ASUSTOR_IP="192.168.0.16"
BINARY_DEST="/home/admin/rust-binaries/photo-server/photo-server"
UI_DEST="/home/admin/rust-binaries/ui/dist/photo-lib/browser"

# Define Rust flags for static linking
export RUSTFLAGS="-C target-feature=+crt-static"

# Navigate to the Rust project directory
echo "Navigating to the Rust project directory..."
cd ./photo-server || exit 1

# Build for aarch64-unknown-linux-gnu
echo "Building for target: aarch64-unknown-linux-gnu with static linking..."
cargo build --target aarch64-unknown-linux-gnu --release --verbose
if [ $? -ne 0 ]; then
    echo "Error: Build failed for target aarch64-unknown-linux-gnu."
    exit 1
fi

# Define the binary path
BINARY="./target/aarch64-unknown-linux-gnu/release/photo_server"

# Check if the binary exists
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary $BINARY not found."
    exit 1
fi

# Stop the running server if it exists
echo "Stopping any running photo-server instance..."
ssh -t -p $SCP_PORT "$ASUSTOR_USER@$ASUSTOR_IP" "pkill photo-server || true"
if [ $? -ne 0 ]; then
    echo "Error: Failed to stop the running photo-server instance."
    exit 1
fi
echo "Successfully stopped any running photo-server instance."

# Set permissions and ownership for the directory and binary
echo "Setting permissions for the photo-server directory and binary..."
ssh -p $SCP_PORT "$ASUSTOR_USER@$ASUSTOR_IP" "
    chmod -R u+rw /home/admin/rust-binaries/photo-server && \
    chown -R root:root /home/admin/rust-binaries/photo-server && \
    rm -f $BINARY_DEST || true
"
if [ $? -ne 0 ]; then
    echo "Error: Failed to set permissions or clean up old binary on the Toaster."
    exit 1
fi
echo "Permissions successfully set and old binary cleaned up."

# Deploy the binary to the Toaster
echo "Deploying $BINARY to $BINARY_DEST on the Toaster..."
scp -P $SCP_PORT "$BINARY" "$ASUSTOR_USER@$ASUSTOR_IP:$BINARY_DEST"
if [ $? -ne 0 ]; then
    echo "Error: Failed to deploy $BINARY to the Toaster."
    exit 1
fi
echo "Successfully deployed $BINARY."

# Build the Angular UI project
echo "Building Angular UI project..."
cd ../ui || exit 1
npm install
npm run build:prod
if [ $? -ne 0 ]; then
    echo "Error: Failed to build Angular UI project."
    exit 1
fi

# Check if the UI build output exists
UI_BUILD_DIR="./dist/photo-lib/browser"
if [ ! -d "$UI_BUILD_DIR" ]; then
    echo "Error: UI build directory $UI_BUILD_DIR not found."
    exit 1
fi

# Create the required directory structure on the Toaster
echo "Creating directory structure on the Toaster..."
ssh -p $SCP_PORT "$ASUSTOR_USER@$ASUSTOR_IP" "mkdir -p $UI_DEST"
if [ $? -ne 0 ]; then
    echo "Error: Failed to create directory structure on the Toaster."
    exit 1
fi

# Deploy the Angular static files to the Toaster
echo "Deploying Angular static files to $UI_DEST on the Toaster..."
scp -r -P $SCP_PORT "$UI_BUILD_DIR"/* "$ASUSTOR_USER@$ASUSTOR_IP:$UI_DEST"
if [ $? -ne 0 ]; then
    echo "Error: Failed to deploy Angular static files to the Toaster."
    exit 1
fi
echo "Successfully deployed Angular UI static files."

# Start the server on the Toaster as root, with debugging and explicit environment setup
echo "Starting the server on the Toaster as root..."
ssh -t -p $SCP_PORT "$ASUSTOR_USER@$ASUSTOR_IP" "
    export OPENSSL_DIR=/usr;
    export OPENSSL_INCLUDE_DIR=/usr/include;
    export OPENSSL_LIB_DIR=/usr/lib;
    cd /home/admin/rust-binaries/photo-server;
    nohup ./photo-server > /home/admin/rust-binaries/photo-server/server.log 2>&1 &
    sleep 5;
    netstat -tuln | grep 443 || echo 'Server not bound to port 443';
"
if [ $? -ne 0 ]; then
    echo "Error: Failed to start the server on the Toaster."
    exit 1
fi

echo "Server successfully started on the Toaster."

echo "All builds and deployments completed successfully!"
