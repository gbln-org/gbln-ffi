#!/usr/bin/env bash
# Build C FFI using Docker for all platforms
# Simpler approach - just use Docker directly

set -e

echo "🔨 Building GBLN C FFI using Docker..."
echo ""

BUILD_START=$(date +%s)

# Function to build in Docker
docker_build() {
    local target=$1
    local platform_name=$2
    local image=$3

    echo "  → $platform_name..."

    # Use cross which internally uses Docker
    if ~/.cargo/bin/cross build --release --target "$target" 2>&1 | tail -3; then
        echo "    ✅ Success"
        return 0
    else
        echo "    ❌ Failed"
        return 1
    fi
}

# Tier 1: FreeBSD
echo "📦 FreeBSD"
docker_build "x86_64-unknown-freebsd" "FreeBSD x86_64" "ghcr.io/cross-rs/x86_64-unknown-freebsd:main"
docker_build "aarch64-unknown-freebsd" "FreeBSD ARM64" "ghcr.io/cross-rs/aarch64-unknown-freebsd:main"

# Tier 1: Linux
echo ""
echo "📦 Linux"
docker_build "x86_64-unknown-linux-gnu" "Linux x86_64" "ghcr.io/cross-rs/x86_64-unknown-linux-gnu:main"
docker_build "aarch64-unknown-linux-gnu" "Linux ARM64" "ghcr.io/cross-rs/aarch64-unknown-linux-gnu:main"

# Tier 1: Windows
echo ""
echo "📦 Windows"
docker_build "x86_64-pc-windows-gnu" "Windows x86_64" "ghcr.io/cross-rs/x86_64-pc-windows-gnu:main"

# Tier 2: Android
echo ""
echo "📦 Android"
docker_build "aarch64-linux-android" "Android ARM64" "ghcr.io/cross-rs/aarch64-linux-android:main"
docker_build "x86_64-linux-android" "Android x86_64" "ghcr.io/cross-rs/x86_64-linux-android:main"

BUILD_END=$(date +%s)
BUILD_TIME=$((BUILD_END - BUILD_START))

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Docker builds complete!"
echo "⏱️  Total time: ${BUILD_TIME}s"
echo ""
echo "📁 Build artifacts in target/<platform>/release/"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
