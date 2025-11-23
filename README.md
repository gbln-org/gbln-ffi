# gbln-ffi - C FFI Layer for GBLN

**Repository**: [gbln-org/gbln-ffi](https://github.com/gbln-org/gbln-ffi)  
**License**: Apache 2.0

---

## What is this?

**This is a C-compatible interface layer for the GBLN parser, implemented in Rust.**

### Why "gbln-ffi" if it's Rust code?

- **What it does**: Exports a **C-compatible API** (C ABI) that any language can use
- **How it works**: Rust code with `extern "C"` functions
- **What it outputs**: 
  - `libgbln.so` / `libgbln.dylib` / `gbln.dll` (C-compatible shared library)
  - `gbln.h` (C header file, auto-generated)
- **Who uses it**: All GBLN language bindings (Python, Go, Swift, Java, etc.)

### Why not pure C?

The core parser is already written in Rust ([gbln-rust](https://github.com/gbln-org/gbln-rust)). This FFI layer:
- **Wraps** the Rust parser with C-compatible functions
- **Uses cbindgen** to auto-generate C headers
- **Provides memory-safe** bindings through Rust's safety guarantees
- **Compiles to** a standard C library that any language can load

**In short**: Rust implementation → C ABI exports → Universal language compatibility

---

## Architecture

```
┌─────────────────┐
│   gbln-rust     │  ← Core parser (Pure Rust)
│   (../rust/)    │
└────────┬────────┘
         │
         │ dependency
         ▼
┌─────────────────┐
│    gbln-ffi     │  ← This repository (Rust with C FFI)
│   (this repo)   │     • extern "C" functions
│                 │     • cbindgen header generation
└────────┬────────┘
         │
         │ outputs
         ▼
┌─────────────────┐
│  C Library      │  ← Platform-specific shared library
│  + C Header     │     • libgbln.so (Linux)
│                 │     • libgbln.dylib (macOS)
│                 │     • gbln.dll (Windows)
│                 │     • gbln.h (C header)
└────────┬────────┘
         │
         │ used by
         ▼
┌─────────────────────────────────────┐
│  All Language Bindings              │
│  • Python (ctypes)                  │
│  • Go (cgo)                         │
│  • Swift (C interop)                │
│  • Java (JNA)                       │
│  • C# (P/Invoke)                    │
│  • ... and 7 more languages         │
└─────────────────────────────────────┘
```

---

## Building

### Requirements

- Rust 1.70+ (`rustup`)
- C compiler (gcc/clang)
- cbindgen (`cargo install cbindgen`)

### Build Shared Library

```bash
# Build for current platform
cargo build --release

# Output locations:
# Linux:   target/release/libgbln.so
# macOS:   target/release/libgbln.dylib
# Windows: target/release/gbln.dll

# C header is auto-generated:
# include/gbln.h
```

### Cross-Platform Builds

```bash
# Linux (from macOS/Windows)
cargo build --release --target x86_64-unknown-linux-gnu

# macOS (from Linux/Windows)
cargo build --release --target x86_64-apple-darwin

# Windows (from Linux/macOS)
cargo build --release --target x86_64-pc-windows-gnu
```

---

## C API Overview

### Core Functions

```c
#include "gbln.h"

// Parse GBLN string
GblnErrorCode gbln_parse(const char* input, GblnValue** out_value);

// Free value
void gbln_value_free(GblnValue* value);

// Serialize (compact)
char* gbln_to_string(const GblnValue* value);

// Serialize (pretty-printed)
char* gbln_to_string_pretty(const GblnValue* value);

// Free string
void gbln_string_free(char* str);
```

### Error Handling

```c
// Get last error message
const char* gbln_last_error_message(void);

// Get error suggestion (if available)
const char* gbln_last_error_suggestion(void);
```

### Value Accessors (All 15 Types)

```c
// Integers
int8_t   gbln_value_as_i8(const GblnValue* value, bool* ok);
int16_t  gbln_value_as_i16(const GblnValue* value, bool* ok);
int32_t  gbln_value_as_i32(const GblnValue* value, bool* ok);
int64_t  gbln_value_as_i64(const GblnValue* value, bool* ok);
uint8_t  gbln_value_as_u8(const GblnValue* value, bool* ok);
uint16_t gbln_value_as_u16(const GblnValue* value, bool* ok);
uint32_t gbln_value_as_u32(const GblnValue* value, bool* ok);
uint64_t gbln_value_as_u64(const GblnValue* value, bool* ok);

// Floats
float  gbln_value_as_f32(const GblnValue* value, bool* ok);
double gbln_value_as_f64(const GblnValue* value, bool* ok);

// String, Bool, Null
const char* gbln_value_as_string(const GblnValue* value, bool* ok);
bool gbln_value_as_bool(const GblnValue* value, bool* ok);
bool gbln_value_is_null(const GblnValue* value);
```

### Object/Array Accessors

```c
// Object access
const GblnValue* gbln_object_get(const GblnValue* value, const char* key);

// Array access
size_t gbln_array_len(const GblnValue* value);
const GblnValue* gbln_array_get(const GblnValue* value, size_t index);
```

---

## Example Usage (C)

```c
#include "gbln.h"
#include <stdio.h>

int main() {
    const char* input = "user{id<u32>(12345)name<s32>(Alice)}";
    GblnValue* value = NULL;
    
    // Parse
    GblnErrorCode err = gbln_parse(input, &value);
    if (err != GBLN_OK) {
        printf("Error: %s\n", gbln_last_error_message());
        return 1;
    }
    
    // Access data
    bool ok;
    const GblnValue* id = gbln_object_get(value, "id");
    uint32_t id_val = gbln_value_as_u32(id, &ok);
    
    if (ok) {
        printf("User ID: %u\n", id_val);
    }
    
    // Serialize
    char* output = gbln_to_string_pretty(value);
    printf("Output:\n%s\n", output);
    
    // Cleanup
    gbln_string_free(output);
    gbln_value_free(value);
    
    return 0;
}
```

**Compile:**
```bash
gcc -o example example.c -L./target/release -lgbln
LD_LIBRARY_PATH=./target/release ./example
```

---

## For Language Binding Authors

### Python (ctypes)

```python
from ctypes import CDLL, c_char_p, POINTER, c_void_p

lib = CDLL("libgbln.so")
lib.gbln_parse.argtypes = [c_char_p, POINTER(c_void_p)]
lib.gbln_parse.restype = c_int

# Use the library...
```

### Go (cgo)

```go
// #cgo LDFLAGS: -lgbln
// #include "gbln.h"
import "C"

func Parse(input string) {
    cInput := C.CString(input)
    defer C.free(unsafe.Pointer(cInput))
    
    var value *C.GblnValue
    err := C.gbln_parse(cInput, &value)
    // ...
}
```

### Swift

```swift
import Foundation

let lib = dlopen("libgbln.dylib", RTLD_NOW)
let parse = unsafeBitCast(
    dlsym(lib, "gbln_parse"),
    to: (@convention(c) (UnsafePointer<CChar>, UnsafeMutablePointer<OpaquePointer?>) -> Int32).self
)
```

**See individual binding repositories for complete examples.**

---

## Testing

### C Integration Tests

```bash
# Run C tests (requires gcc/clang)
cd tests
gcc -o ffi_test ffi_test.c -L../target/release -lgbln
LD_LIBRARY_PATH=../target/release ./ffi_test
```

### Memory Safety (Valgrind)

```bash
# Check for memory leaks
valgrind --leak-check=full ./ffi_test
```

Expected: **0 bytes lost, 0 errors**

---

## Project Structure

```
core/ffi/ (gbln-ffi repository)
├── Cargo.toml              # Rust build configuration
├── build.rs                # cbindgen header generation
├── src/
│   ├── lib.rs              # Main FFI exports
│   ├── types.rs            # C-compatible types
│   ├── error.rs            # Error handling
│   └── accessors.rs        # Value accessor functions
├── include/
│   └── gbln.h              # Generated C header (auto)
├── tests/
│   └── ffi_test.c          # C integration tests
├── examples/
│   └── example.c           # Example C usage
└── README.md               # This file
```

---

## Dependencies

- **gbln-rust** ([../rust/](../rust/)) - Core GBLN parser
- **cbindgen** - Automatic C header generation

---

## License

Apache License 2.0 - See [LICENSE](LICENSE)

---

## See Also

- **[GBLN Specification](../../docs/01-specification.md)** - Format specification
- **[gbln-rust](../rust/)** - Core Rust implementation
- **[Language Bindings](../../bindings/)** - Python, Go, Swift, etc.
- **[Main Repository](../../)** - GBLN ecosystem hub

---

**Questions?** Open an issue at [github.com/gbln-org/gbln-ffi](https://github.com/gbln-org/gbln-ffi)
