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

    pub fn into_inner(self) -> Value {
        self.inner
    }
}

/// Value type enum for C FFI
///
/// Represents the type of a GBLN value.
/// This allows C code to query the type without trying every accessor.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GblnValueType {
    I8 = 0,
    I16 = 1,
    I32 = 2,
    I64 = 3,
    U8 = 4,
    U16 = 5,
    U32 = 6,
    U64 = 7,
    F32 = 8,
    F64 = 9,
    Str = 10,
    Bool = 11,
    Null = 12,
    Object = 13,
    Array = 14,
}

impl From<&Value> for GblnValueType {
    fn from(value: &Value) -> Self {
        match value {
            Value::I8(_) => GblnValueType::I8,
            Value::I16(_) => GblnValueType::I16,
            Value::I32(_) => GblnValueType::I32,
            Value::I64(_) => GblnValueType::I64,
            Value::U8(_) => GblnValueType::U8,
            Value::U16(_) => GblnValueType::U16,
            Value::U32(_) => GblnValueType::U32,
            Value::U64(_) => GblnValueType::U64,
            Value::F32(_) => GblnValueType::F32,
            Value::F64(_) => GblnValueType::F64,
            Value::Str(_) => GblnValueType::Str,
            Value::Bool(_) => GblnValueType::Bool,
            Value::Null => GblnValueType::Null,
            Value::Object(_) => GblnValueType::Object,
            Value::Array(_) => GblnValueType::Array,
        }
    }
}
