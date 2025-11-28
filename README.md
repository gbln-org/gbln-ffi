# GBLN C FFI

This directory contains the C Foreign Function Interface (FFI) layer for GBLN.

## Build System

**IMPORTANT:** C FFI libraries are built **INSIDE Docker containers** as part of language binding tests.

**There are NO standalone build scripts here.**

### Why?

Each language binding builds the C FFI it needs inside its Docker test container:

```
bindings/{language}/tests/docker/Dockerfile.{platform}
```

Each Dockerfile:
1. Installs Rust
2. Builds C FFI for that platform
3. Copies library to standard location
4. Tests the binding

**This ensures:** Build + Test happen in the same environment.

## For Language Binding Development

If you need to test C FFI alone:

```bash
# Example: Linux x86_64
docker run --rm -v $(pwd):/build -w /build rust:1.87-slim sh -c "
  rustup target add x86_64-unknown-linux-gnu
  cargo build --release --target x86_64-unknown-linux-gnu
"
```

But normally, just run the binding tests:

```bash
cd bindings/python/tests/docker
./test-all-platforms.sh
```

## Documentation

See `docs/builds/BUILD_SYSTEM.md` for complete build system documentation.

---

**Build Method:** Docker-only (no standalone scripts)  
**Last Updated:** 2025-11-27
