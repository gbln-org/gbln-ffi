// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

///! Extended C FFI functions for language bindings
///!
///! These functions provide:
///! - Type introspection (`gbln_value_type`)
///! - Object iteration (`gbln_object_keys`, `gbln_object_len`)
///! - Value construction (`gbln_value_new_*`)
///! - Object/array building (`gbln_object_insert`, `gbln_array_push`)
use crate::error::{set_last_error, GblnErrorCode};
use crate::types::{GblnValue, GblnValueType};
use gbln::Value;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

// ============================================================================
// Type Introspection
// ============================================================================

/// Get value type
///
/// Returns the type of a GBLN value.
/// Eliminates need to try every as_* function.
///
/// # Safety
/// - `value` must be a valid GblnValue pointer
#[no_mangle]
pub extern "C" fn gbln_value_type(value: *const GblnValue) -> GblnValueType {
    if value.is_null() {
        return GblnValueType::Null;
    }

    let value_ref = unsafe { (*value).inner() };
    GblnValueType::from(value_ref)
}

// ============================================================================
// Object Iteration
// ============================================================================

/// Get number of fields in object
///
/// Returns 0 if value is not an object.
///
/// # Safety
/// - `value` must be a valid GblnValue pointer
#[no_mangle]
pub extern "C" fn gbln_object_len(value: *const GblnValue) -> usize {
    if value.is_null() {
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::Object(map) => map.len(),
        _ => 0,
    }
}

/// Get object keys
///
/// Returns array of null-terminated C strings.
/// Caller must free with `gbln_keys_free()`.
///
/// Returns NULL if value is not an object.
///
/// # Safety
/// - `value` must be a valid GblnValue pointer
/// - `out_count` must be a valid pointer to store the count
/// - Caller must free returned array with `gbln_keys_free()`
#[no_mangle]
pub extern "C" fn gbln_object_keys(
    value: *const GblnValue,
    out_count: *mut usize,
) -> *mut *mut c_char {
    if value.is_null() || out_count.is_null() {
        return ptr::null_mut();
    }

    match unsafe { (*value).inner() } {
        Value::Object(map) => {
            let keys: Vec<*mut c_char> = map
                .keys()
                .filter_map(|k| CString::new(k.as_str()).ok())
                .map(|s| s.into_raw())
                .collect();

            let count = keys.len();
            unsafe {
                *out_count = count;
            }

            if count == 0 {
                return ptr::null_mut();
            }

            // Allocate array of C string pointers
            let mut boxed = keys.into_boxed_slice();
            let ptr = boxed.as_mut_ptr();
            std::mem::forget(boxed);
            ptr
        }
        _ => {
            unsafe {
                *out_count = 0;
            }
            ptr::null_mut()
        }
    }
}

/// Free keys array
///
/// Frees array returned by `gbln_object_keys()`.
///
/// # Safety
/// - `keys` must be a valid pointer returned from `gbln_object_keys()`
/// - `count` must be the count returned from `gbln_object_keys()`
/// - Must not be called twice on the same pointer
#[no_mangle]
pub extern "C" fn gbln_keys_free(keys: *mut *mut c_char, count: usize) {
    if keys.is_null() || count == 0 {
        return;
    }

    unsafe {
        // Free each individual string
        for i in 0..count {
            let key_ptr = *keys.add(i);
            if !key_ptr.is_null() {
                drop(CString::from_raw(key_ptr));
            }
        }

        // Free the array itself
        drop(Box::from_raw(std::slice::from_raw_parts_mut(keys, count)));
    }
}

// ============================================================================
// Value Constructors - Integers
// ============================================================================

/// Create i8 value
#[no_mangle]
pub extern "C" fn gbln_value_new_i8(value: i8) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::I8(value))))
}

/// Create i16 value
#[no_mangle]
pub extern "C" fn gbln_value_new_i16(value: i16) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::I16(value))))
}

/// Create i32 value
#[no_mangle]
pub extern "C" fn gbln_value_new_i32(value: i32) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::I32(value))))
}

/// Create i64 value
#[no_mangle]
pub extern "C" fn gbln_value_new_i64(value: i64) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::I64(value))))
}

/// Create u8 value
#[no_mangle]
pub extern "C" fn gbln_value_new_u8(value: u8) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::U8(value))))
}

/// Create u16 value
#[no_mangle]
pub extern "C" fn gbln_value_new_u16(value: u16) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::U16(value))))
}

/// Create u32 value
#[no_mangle]
pub extern "C" fn gbln_value_new_u32(value: u32) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::U32(value))))
}

/// Create u64 value
#[no_mangle]
pub extern "C" fn gbln_value_new_u64(value: u64) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::U64(value))))
}

// ============================================================================
// Value Constructors - Floats
// ============================================================================

/// Create f32 value
#[no_mangle]
pub extern "C" fn gbln_value_new_f32(value: f32) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::F32(value))))
}

/// Create f64 value
#[no_mangle]
pub extern "C" fn gbln_value_new_f64(value: f64) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::F64(value))))
}

// ============================================================================
// Value Constructors - String, Bool, Null
// ============================================================================

/// Create string value
///
/// # Args
/// - value: null-terminated UTF-8 string
/// - max_len: maximum string length (for type hint)
///
/// # Returns
/// - GblnValue pointer on success
/// - NULL if string exceeds max_len or invalid UTF-8
///
/// # Safety
/// - `value` must be a valid null-terminated UTF-8 string
#[no_mangle]
pub extern "C" fn gbln_value_new_str(value: *const c_char, max_len: usize) -> *mut GblnValue {
    if value.is_null() {
        set_last_error("Null pointer for string value".to_string(), None);
        return ptr::null_mut();
    }

    let value_str = unsafe {
        match CStr::from_ptr(value).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8: {}", e), None);
                return ptr::null_mut();
            }
        }
    };

    // Check length (character count, not bytes)
    let char_count = value_str.chars().count();
    if char_count > max_len {
        set_last_error(
            format!("String too long: {} chars (max: {})", char_count, max_len),
            Some(format!(
                "Use a larger string type (s{} or larger)",
                max_len * 2
            )),
        );
        return ptr::null_mut();
    }

    Box::into_raw(Box::new(GblnValue::new(Value::Str(value_str.to_string()))))
}

/// Create boolean value
#[no_mangle]
pub extern "C" fn gbln_value_new_bool(value: bool) -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::Bool(value))))
}

/// Create null value
#[no_mangle]
pub extern "C" fn gbln_value_new_null() -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::Null)))
}

// ============================================================================
// Object Building
// ============================================================================

/// Create empty object
#[no_mangle]
pub extern "C" fn gbln_value_new_object() -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::Object(HashMap::new()))))
}

/// Insert field into object
///
/// # Safety
/// - `object` must be a GblnValue of type Object
/// - `key` must be a valid null-terminated UTF-8 string
/// - `value` ownership is transferred to the object
///
/// # Returns
/// - GBLN_OK on success
/// - GBLN_ERROR_DUPLICATE_KEY if key already exists
/// - GBLN_ERROR_TYPE_MISMATCH if object is not an Object type
/// - GBLN_ERROR_NULL_POINTER if any pointer is null
#[no_mangle]
pub extern "C" fn gbln_object_insert(
    object: *mut GblnValue,
    key: *const c_char,
    value: *mut GblnValue,
) -> GblnErrorCode {
    if object.is_null() || key.is_null() || value.is_null() {
        set_last_error("Null pointer in object_insert".to_string(), None);
        return GblnErrorCode::ErrorNullPointer;
    }

    let key_str = unsafe {
        match CStr::from_ptr(key).to_str() {
            Ok(s) => s.to_string(),
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in key: {}", e), None);
                return GblnErrorCode::ErrorNullPointer;
            }
        }
    };

    // Take ownership of value
    let value_box = unsafe { Box::from_raw(value) };
    let value_inner = value_box.into_inner();

    // Get mutable reference to object
    let object_ref = unsafe { &mut *object };

    match object_ref.inner() {
        Value::Object(_) => {
            // We need to get mutable access to the inner HashMap
            // This requires unsafe access to the inner field
            let inner_ptr = object_ref as *mut GblnValue as *mut Value;
            let inner_ref = unsafe { &mut *inner_ptr };

            if let Value::Object(ref mut map) = inner_ref {
                if map.contains_key(&key_str) {
                    set_last_error(
                        format!("Duplicate key: {}", key_str),
                        Some("Use a different key name".to_string()),
                    );
                    return GblnErrorCode::ErrorDuplicateKey;
                }

                map.insert(key_str, value_inner);
                GblnErrorCode::Ok
            } else {
                unreachable!()
            }
        }
        _ => {
            set_last_error(
                "Value is not an object".to_string(),
                Some("Use gbln_value_new_object() to create an object".to_string()),
            );
            GblnErrorCode::ErrorTypeMismatch
        }
    }
}

// ============================================================================
// Array Building
// ============================================================================

/// Create empty array
#[no_mangle]
pub extern "C" fn gbln_value_new_array() -> *mut GblnValue {
    Box::into_raw(Box::new(GblnValue::new(Value::Array(Vec::new()))))
}

/// Push value to array
///
/// # Safety
/// - `array` must be a GblnValue of type Array
/// - `value` ownership is transferred to the array
///
/// # Returns
/// - GBLN_OK on success
/// - GBLN_ERROR_TYPE_MISMATCH if array is not an Array type
/// - GBLN_ERROR_NULL_POINTER if any pointer is null
#[no_mangle]
pub extern "C" fn gbln_array_push(array: *mut GblnValue, value: *mut GblnValue) -> GblnErrorCode {
    if array.is_null() || value.is_null() {
        set_last_error("Null pointer in array_push".to_string(), None);
        return GblnErrorCode::ErrorNullPointer;
    }

    // Take ownership of value
    let value_box = unsafe { Box::from_raw(value) };
    let value_inner = value_box.into_inner();

    // Get mutable reference to array
    let array_ref = unsafe { &mut *array };

    match array_ref.inner() {
        Value::Array(_) => {
            // Get mutable access to the inner Vec
            let inner_ptr = array_ref as *mut GblnValue as *mut Value;
            let inner_ref = unsafe { &mut *inner_ptr };

            if let Value::Array(ref mut vec) = inner_ref {
                vec.push(value_inner);
                GblnErrorCode::Ok
            } else {
                unreachable!()
            }
        }
        _ => {
            set_last_error(
                "Value is not an array".to_string(),
                Some("Use gbln_value_new_array() to create an array".to_string()),
            );
            GblnErrorCode::ErrorTypeMismatch
        }
    }
}
