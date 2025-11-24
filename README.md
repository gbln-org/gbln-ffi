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

// Parse from file
GblnErrorCode gbln_parse_file(const char* path, GblnValue** out_value);

// Free value
void gbln_value_free(GblnValue* value);

// Serialize (MINI GBLN - default)
char* gbln_to_string(const GblnValue* value);

// Serialize (pretty-printed)
char* gbln_to_string_pretty(const GblnValue* value);

// Free string
void gbln_string_free(char* str);
```

### I/O Format Functions

```c
// Configuration for I/O format
typedef struct {
    bool mini_mode;           // Enable MINI GBLN (no structural whitespace)
    bool compress;            // Enable XZ compression
    uint8_t compression_level; // XZ compression level (0-9)
    size_t indent;            // Indentation width for pretty printing
    bool strip_comments;      // Strip comments in I/O format
} GblnConfig;

// Create default configuration (MINI + compression)
GblnConfig gbln_config_default(void);

// Create development configuration (pretty, no compression)
GblnConfig gbln_config_development(void);

// Create I/O format configuration (MINI + XZ compression)
GblnConfig gbln_config_io_format(void);

// Write I/O format to file
GblnErrorCode gbln_write_io(
    const GblnValue* value,
    const char* path,
    const GblnConfig* config
);

// Read I/O format from file
GblnErrorCode gbln_read_io(
    const char* path,
    GblnValue** out_value
);
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

### Basic Parsing

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
    const GblnValue* user = gbln_object_get(value, "user");
    const GblnValue* id = gbln_object_get(user, "id");
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

### I/O Format Workflow

```c
#include "gbln.h"
#include <stdio.h>

int main() {
    // 1. DEVELOPMENT: Parse source file
    GblnValue* value = NULL;
    GblnErrorCode err = gbln_parse_file("config.gbln", &value);
    if (err != GBLN_OK) {
        printf("Error: %s\n", gbln_last_error_message());
        return 1;
    }
    printf("✓ Parsed config.gbln\n");
    
    // 2. PRODUCTION: Generate I/O format
    GblnConfig io_config = gbln_config_io_format();
    err = gbln_write_io(value, "config.io.gbln.xz", &io_config);
    if (err != GBLN_OK) {
        printf("Error writing I/O: %s\n", gbln_last_error_message());
        gbln_value_free(value);
        return 1;
    }
    printf("✓ Generated config.io.gbln.xz (MINI + XZ compressed)\n");
    
    // 3. Optional: Generate debug format (MINI only, no compression)
    GblnConfig debug_config = gbln_config_default();
    debug_config.compress = false;
    err = gbln_write_io(value, "config.io.gbln", &debug_config);
    if (err == GBLN_OK) {
        printf("✓ Generated config.io.gbln (MINI, uncompressed)\n");
    }
    
    gbln_value_free(value);
    
    // 4. APPLICATION: Load from I/O format
    GblnValue* loaded = NULL;
    err = gbln_read_io("config.io.gbln.xz", &loaded);
    if (err != GBLN_OK) {
        printf("Error reading I/O: %s\n", gbln_last_error_message());
        return 1;
    }
    printf("✓ Loaded from config.io.gbln.xz\n");
    
    // 5. Use the configuration
    bool ok;
    const GblnValue* app = gbln_object_get(loaded, "app");
    const GblnValue* server = gbln_object_get(app, "server");
    const GblnValue* port = gbln_object_get(server, "port");
    uint16_t port_val = gbln_value_as_u16(port, &ok);
    
    if (ok) {
        printf("✓ Server will run on port %u\n", port_val);
    }
    
    // Cleanup
    gbln_value_free(loaded);
    
    return 0;
}
```

**Compile:**
```bash
gcc -o example example.c -L./target/release -lgbln
LD_LIBRARY_PATH=./target/release ./example
```

**Output:**
```
✓ Parsed config.gbln
✓ Generated config.io.gbln.xz (MINI + XZ compressed)
✓ Generated config.io.gbln (MINI, uncompressed)
✓ Loaded from config.io.gbln.xz
✓ Server will run on port 8080
```

---

## For Language Binding Authors

### Python (ctypes)

```python
from ctypes import CDLL, c_char_p, POINTER, c_void_p, Structure, c_bool, c_uint8, c_size_t

lib = CDLL("libgbln.so")

# Basic parsing
lib.gbln_parse.argtypes = [c_char_p, POINTER(c_void_p)]
lib.gbln_parse.restype = c_int

# I/O format configuration
class GblnConfig(Structure):
    _fields_ = [
        ("mini_mode", c_bool),
        ("compress", c_bool),
        ("compression_level", c_uint8),
        ("indent", c_size_t),
        ("strip_comments", c_bool),
    ]

lib.gbln_config_io_format.restype = GblnConfig
lib.gbln_write_io.argtypes = [c_void_p, c_char_p, POINTER(GblnConfig)]
lib.gbln_write_io.restype = c_int
lib.gbln_read_io.argtypes = [c_char_p, POINTER(c_void_p)]
lib.gbln_read_io.restype = c_int

# Example usage
config = lib.gbln_config_io_format()
value = c_void_p()
lib.gbln_parse_file(b"config.gbln", byref(value))
lib.gbln_write_io(value, b"config.io.gbln.xz", byref(config))
```

### Go (cgo)

```go
// #cgo LDFLAGS: -lgbln
// #include "gbln.h"
import "C"
import "unsafe"

func Parse(input string) (*C.GblnValue, error) {
    cInput := C.CString(input)
    defer C.free(unsafe.Pointer(cInput))
    
    var value *C.GblnValue
    err := C.gbln_parse(cInput, &value)
    if err != C.GBLN_OK {
        return nil, errors.New(C.GoString(C.gbln_last_error_message()))
    }
    return value, nil
}

func WriteIO(value *C.GblnValue, path string) error {
    cPath := C.CString(path)
    defer C.free(unsafe.Pointer(cPath))
    
    config := C.gbln_config_io_format()
    err := C.gbln_write_io(value, cPath, &config)
    if err != C.GBLN_OK {
        return errors.New(C.GoString(C.gbln_last_error_message()))
    }
    return nil
}

func ReadIO(path string) (*C.GblnValue, error) {
    cPath := C.CString(path)
    defer C.free(unsafe.Pointer(cPath))
    
    var value *C.GblnValue
    err := C.gbln_read_io(cPath, &value)
    if err != C.GBLN_OK {
        return nil, errors.New(C.GoString(C.gbln_last_error_message()))
    }
    return value, nil
}
```

### Swift

```swift
import Foundation

typealias GblnConfig = (
    mini_mode: Bool,
    compress: Bool,
    compression_level: UInt8,
    indent: Int,
    strip_comments: Bool
)

class GBLN {
    let lib: UnsafeMutableRawPointer
    
    init() {
        lib = dlopen("libgbln.dylib", RTLD_NOW)!
    }
    
    func writeIO(value: OpaquePointer, path: String, config: GblnConfig) -> Bool {
        let writeIO = unsafeBitCast(
            dlsym(lib, "gbln_write_io"),
            to: (@convention(c) (OpaquePointer, UnsafePointer<CChar>, UnsafePointer<GblnConfig>) -> Int32).self
        )
        
        return path.withCString { cPath in
            var cfg = config
            return withUnsafePointer(to: &cfg) { configPtr in
                writeIO(value, cPath, configPtr) == 0
            }
        }
    }
    
    func readIO(path: String) -> OpaquePointer? {
        let readIO = unsafeBitCast(
            dlsym(lib, "gbln_read_io"),
            to: (@convention(c) (UnsafePointer<CChar>, UnsafeMutablePointer<OpaquePointer?>) -> Int32).self
        )
        
        var value: OpaquePointer?
        path.withCString { cPath in
            _ = readIO(cPath, &value)
        }
        return value
    }
}
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
│   ├── config.rs           # GblnConfig FFI wrapper
│   ├── error.rs            # Error handling
│   ├── io.rs               # I/O format read/write
│   └── accessors.rs        # Value accessor functions
├── include/
│   └── gbln.h              # Generated C header (auto)
├── tests/
│   ├── ffi_test.c          # C integration tests
│   └── io_test.c           # I/O format tests
├── examples/
│   ├── example.c           # Basic parsing example
│   └── io_example.c        # I/O format workflow example
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
