#!/bin/sh

echo "Build release..."
cargo build --release --features release

# echo "Prepare release..."
# mkdir -p target/release/assets
# cp -r assets target/release/assets

echo "Creating installer..."
dotnet build -c Release build/windows/installer/Installer.wixproj --output installer