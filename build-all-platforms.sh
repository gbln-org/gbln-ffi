#!/usr/bin/env bash
# Build C FFI for all platforms (Ticket #005B)
# Based on build script from Ticket #004C
# Requires bash 4+ for associative arrays

set -e

# Check bash version
if [ "${BASH_VERSINFO[0]}" -lt 4 ]; then
    echo "Error: This script requires bash 4 or later"
    echo "Current version: $BASH_VERSION"
    echo "Install via: brew install bash"
    exit 1
fi

echo "🔨 Building GBLN C FFI for all platforms..."
echo ""

# Ensure we're using the right Rust version
export RUSTUP_TOOLCHAIN=1.91.1

# Build timestamp
BUILD_START=$(date +%s)

# Results tracking
declare -A RESULTS
declare -A SIZES

# Tier 1: FreeBSD (x86_64, arm64)
echo "📦 Tier 1: FreeBSD"

echo "  → FreeBSD x86_64..."
if ~/.cargo/bin/cross build --release --target x86_64-unknown-freebsd 2>&1 | tail -5; then
    RESULTS[freebsd-x64]="✅"
    SIZES[freebsd-x64]=$(ls -lh target/x86_64-unknown-freebsd/release/libgbln.so 2>/dev/null | awk '{print $5}')
else
    RESULTS[freebsd-x64]="❌"
fi

echo "  → FreeBSD ARM64..."
if ~/.cargo/bin/cross +nightly build --release --target aarch64-unknown-freebsd 2>&1 | tail -5; then
    RESULTS[freebsd-arm64]="✅"
    SIZES[freebsd-arm64]=$(ls -lh target/aarch64-unknown-freebsd/release/libgbln.so 2>/dev/null | awk '{print $5}')
else
    RESULTS[freebsd-arm64]="❌"
fi

# Tier 1: Linux (x86_64, arm64)
echo ""
echo "📦 Tier 1: Linux"

echo "  → Linux x86_64..."
if ~/.cargo/bin/cross build --release --target x86_64-unknown-linux-gnu 2>&1 | tail -5; then
    RESULTS[linux-x64]="✅"
    SIZES[linux-x64]=$(ls -lh target/x86_64-unknown-linux-gnu/release/libgbln.so 2>/dev/null | awk '{print $5}')
else
    RESULTS[linux-x64]="❌"
fi

echo "  → Linux ARM64..."
if ~/.cargo/bin/cross build --release --target aarch64-unknown-linux-gnu 2>&1 | tail -5; then
    RESULTS[linux-arm64]="✅"
    SIZES[linux-arm64]=$(ls -lh target/aarch64-unknown-linux-gnu/release/libgbln.so 2>/dev/null | awk '{print $5}')
else
    RESULTS[linux-arm64]="❌"
fi

# Tier 1: macOS (x86_64, arm64)
echo ""
echo "📦 Tier 1: macOS"

echo "  → macOS x86_64..."
if cargo build --release --target x86_64-apple-darwin 2>&1 | tail -5; then
    RESULTS[macos-x64]="✅"
    SIZES[macos-x64]=$(ls -lh target/x86_64-apple-darwin/release/libgbln.dylib 2>/dev/null | awk '{print $5}')
else
    RESULTS[macos-x64]="❌"
fi

echo "  → macOS ARM64..."
if cargo build --release --target aarch64-apple-darwin 2>&1 | tail -5; then
    RESULTS[macos-arm64]="✅"
    SIZES[macos-arm64]=$(ls -lh target/aarch64-apple-darwin/release/libgbln.dylib 2>/dev/null | awk '{print $5}')
else
    RESULTS[macos-arm64]="❌"
fi

# Tier 1: Windows (x86_64)
echo ""
echo "📦 Tier 1: Windows"

echo "  → Windows x86_64..."
if ~/.cargo/bin/cross build --release --target x86_64-pc-windows-gnu 2>&1 | tail -5; then
    RESULTS[windows-x64]="✅"
    SIZES[windows-x64]=$(ls -lh target/x86_64-pc-windows-gnu/release/gbln.dll 2>/dev/null | awk '{print $5}')
else
    RESULTS[windows-x64]="❌"
fi

# Tier 2: Mobile (iOS, Android)
echo ""
echo "📦 Tier 2: Mobile"

echo "  → iOS ARM64..."
if cargo build --release --target aarch64-apple-ios 2>&1 | tail -5; then
    RESULTS[ios-arm64]="✅"
    SIZES[ios-arm64]=$(ls -lh target/aarch64-apple-ios/release/libgbln.a 2>/dev/null | awk '{print $5}')
else
    RESULTS[ios-arm64]="❌"
fi

echo "  → Android ARM64..."
if ~/.cargo/bin/cross build --release --target aarch64-linux-android 2>&1 | tail -5; then
    RESULTS[android-arm64]="✅"
    SIZES[android-arm64]=$(ls -lh target/aarch64-linux-android/release/libgbln.so 2>/dev/null | awk '{print $5}')
else
    RESULTS[android-arm64]="❌"
fi

echo "  → Android x86_64..."
if ~/.cargo/bin/cross build --release --target x86_64-linux-android 2>&1 | tail -5; then
    RESULTS[android-x64]="✅"
    SIZES[android-x64]=$(ls -lh target/x86_64-linux-android/release/libgbln.so 2>/dev/null | awk '{print $5}')
else
    RESULTS[android-x64]="❌"
fi

# Calculate build time
BUILD_END=$(date +%s)
BUILD_TIME=$((BUILD_END - BUILD_START))

# Print results
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Build Results (Ticket #005B - C FFI Extensions)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
printf "%-20s %-8s %-10s\n" "Platform" "Status" "Size"
echo "────────────────────────────────────────────────────"
printf "%-20s %-8s %-10s\n" "FreeBSD x86_64" "${RESULTS[freebsd-x64]}" "${SIZES[freebsd-x64]}"
printf "%-20s %-8s %-10s\n" "FreeBSD ARM64" "${RESULTS[freebsd-arm64]}" "${SIZES[freebsd-arm64]}"
printf "%-20s %-8s %-10s\n" "Linux x86_64" "${RESULTS[linux-x64]}" "${SIZES[linux-x64]}"
printf "%-20s %-8s %-10s\n" "Linux ARM64" "${RESULTS[linux-arm64]}" "${SIZES[linux-arm64]}"
printf "%-20s %-8s %-10s\n" "macOS x86_64" "${RESULTS[macos-x64]}" "${SIZES[macos-x64]}"
printf "%-20s %-8s %-10s\n" "macOS ARM64" "${RESULTS[macos-arm64]}" "${SIZES[macos-arm64]}"
printf "%-20s %-8s %-10s\n" "Windows x86_64" "${RESULTS[windows-x64]}" "${SIZES[windows-x64]}"
printf "%-20s %-8s %-10s\n" "iOS ARM64" "${RESULTS[ios-arm64]}" "${SIZES[ios-arm64]}"
printf "%-20s %-8s %-10s\n" "Android ARM64" "${RESULTS[android-arm64]}" "${SIZES[android-arm64]}"
printf "%-20s %-8s %-10s\n" "Android x86_64" "${RESULTS[android-x64]}" "${SIZES[android-x64]}"
echo ""
echo "⏱️  Total build time: ${BUILD_TIME}s"
echo ""

# Check if all succeeded
FAILED=0
for platform in "${!RESULTS[@]}"; do
    if [[ "${RESULTS[$platform]}" == "❌" ]]; then
        FAILED=1
    fi
done

if [ $FAILED -eq 0 ]; then
    echo "✅ All platforms built successfully!"
    exit 0
else
    echo "❌ Some platforms failed. See above for details."
    exit 1
fi
