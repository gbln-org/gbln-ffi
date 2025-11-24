use gbln::Value;

/// Opaque pointer to a GBLN value
///
/// This is a C-compatible wrapper around Rust's Value type.
/// C code sees this as an opaque pointer and must use the provided
/// accessor functions to interact with it.
///
/// Note: This struct is intentionally opaque to C. The internal
/// Value type is not exposed in the C header.
pub struct GblnValue {
    inner: Value,
}

impl GblnValue {
    pub fn new(value: Value) -> Self {
        GblnValue { inner: value }
    }

    pub fn inner(&self) -> &Value {
        &self.inner
    }
}
