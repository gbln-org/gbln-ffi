#!/usr/bin/env bash
#
# GBLN C FFI - Complete Platform Build Script
#
# THIS IS THE ONLY VALID WAY TO BUILD C FFI LIBRARIES FOR ALL PLATFORMS.
#
# CRITICAL: Builds MUST be sequential to avoid Cargo lock file conflicts.
# Parallel builds will hang waiting for locks. This has been proven multiple times.
#
# Usage:
#   ./build-all.sh
#
# Prerequisites:
#   - Docker installed and running
#   - rustup installed
#   - cross tool installed: cargo install cross --git https://github.com/cross-rs/cross
#   - nightly toolchain installed: rustup install nightly
#
# Output:
#   - All 8 platform libraries in target/{triple}/release/
#   - Organized copies in libs/{platform}/
#
# DO NOT MODIFY THIS SCRIPT WITHOUT TESTING ALL 8 PLATFORMS.
# DO NOT RUN BUILDS IN PARALLEL.
#

set -euo pipefail

# Ensure we're in the correct directory
cd "$(dirname "$0")"

# Ensure PATH includes cargo binaries
export PATH="$HOME/.cargo/bin:$PATH"

echo "=================================================="
echo "GBLN C FFI - Building All Platforms"
echo "=================================================="
echo ""
echo "⚠️  IMPORTANT: Builds run SEQUENTIALLY to avoid lock conflicts"
echo "⏱️  Estimated time: 5-10 minutes total"
echo ""

# Counter for progress
BUILT=0
TOTAL=8

build_platform() {
    local target=$1
    local name=$2
    local extra_args=$3

    BUILT=$((BUILT + 1))
    echo "[$BUILT/$TOTAL] Building: $name ($target)"
    echo "-------------------------------------------"

    if [ -n "$extra_args" ]; then
        # shellcheck disable=SC2086
        cross $extra_args build --release --target "$target"
    else
        cross build --release --target "$target"
    fi

    echo "✅ $name complete"
    echo ""
}

# Linux x86_64
build_platform "x86_64-unknown-linux-gnu" "Linux x86_64" ""

# Linux ARM64
build_platform "aarch64-unknown-linux-gnu" "Linux ARM64" ""

# FreeBSD x86_64
build_platform "x86_64-unknown-freebsd" "FreeBSD x86_64" ""

# FreeBSD ARM64 (requires nightly + build-std)
build_platform "aarch64-unknown-freebsd" "FreeBSD ARM64 (nightly)" "+nightly -Z build-std"

# Windows x86_64
build_platform "x86_64-pc-windows-gnu" "Windows x86_64" ""

# Android ARM64
build_platform "aarch64-linux-android" "Android ARM64" ""

# Android x86_64
build_platform "x86_64-linux-android" "Android x86_64" ""

# macOS ARM64 (native, not cross)
echo "[$((BUILT + 1))/$TOTAL] Building: macOS ARM64 (aarch64-apple-darwin)"
echo "-------------------------------------------"
cargo build --release --target aarch64-apple-darwin
echo "✅ macOS ARM64 complete"
echo ""

echo "=================================================="
echo "✅ ALL 8 PLATFORMS BUILT SUCCESSFULLY"
echo "=================================================="
echo ""
echo "Organizing libraries into libs/ directory..."

# Create directory structure
mkdir -p libs/{linux-x64,linux-arm64,freebsd-x64,freebsd-arm64,windows-x64,android-arm64,android-x64,macos-arm64}

# Copy libraries to organized structure
cp target/x86_64-unknown-linux-gnu/release/libgbln.so libs/linux-x64/
cp target/aarch64-unknown-linux-gnu/release/libgbln.so libs/linux-arm64/
cp target/x86_64-unknown-freebsd/release/libgbln.so libs/freebsd-x64/
cp target/aarch64-unknown-freebsd/release/libgbln.so libs/freebsd-arm64/
cp target/x86_64-pc-windows-gnu/release/gbln.dll libs/windows-x64/
cp target/aarch64-linux-android/release/libgbln.so libs/android-arm64/
cp target/x86_64-linux-android/release/libgbln.so libs/android-x64/
cp target/aarch64-apple-darwin/release/libgbln.dylib libs/macos-arm64/

echo "✅ Libraries organized in libs/"
echo ""

# Display library sizes
echo "Library sizes:"
echo "-------------------------------------------"
du -h libs/*/* | sed 's|libs/||'
echo ""

echo "=================================================="
echo "BUILD COMPLETE"
echo "=================================================="
echo ""
echo "Next steps:"
echo "  1. Verify libraries: find libs -type f"
echo "  2. Commit to Git: git add libs/ && git commit -m '...'"
echo "  3. Test bindings: cd ../../bindings/python && pytest"
echo ""
echo "All libraries ready for distribution in language bindings."
