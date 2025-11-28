// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

use gbln::{parse, to_string, to_string_pretty, Value};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

mod accessors;
mod error;
mod extensions;
mod types;

pub use error::{get_last_error, set_last_error, GblnErrorCode};
pub use types::{GblnValue, GblnValueType};

/// Parse GBLN string into a value
///
/// # Safety
/// - `input` must be a valid null-terminated UTF-8 string
/// - `out_value` must be a valid pointer to store the result
/// - Caller must free the returned value with `gbln_value_free()`
///
/// # Returns
/// - `GBLN_OK` on success, with `out_value` set to the parsed value
/// - Error code on failure, with error details available via `gbln_last_error_message()`
#[no_mangle]
pub extern "C" fn gbln_parse(
    input: *const c_char,
    out_value: *mut *mut GblnValue,
) -> GblnErrorCode {
    if input.is_null() || out_value.is_null() {
        set_last_error("Null pointer".to_string(), None);
        return GblnErrorCode::ErrorNullPointer;
    }

    let input_str = unsafe {
        match CStr::from_ptr(input).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8: {}", e), None);
                return GblnErrorCode::ErrorNullPointer;
            }
        }
    };

    match parse(input_str) {
        Ok(value) => {
            let boxed = Box::new(GblnValue::new(value));
            unsafe {
                *out_value = Box::into_raw(boxed);
            }
            GblnErrorCode::Ok
        }
        Err(e) => {
            let suggestion = e.suggestion.clone();
            set_last_error(e.to_string(), suggestion);
            error::map_error_kind(&e.kind)
        }
    }
}

/// Free a GBLN value
///
/// # Safety
/// - `value` must be a valid pointer returned from `gbln_parse()` or NULL
/// - Must not be called twice on the same pointer
#[no_mangle]
pub extern "C" fn gbln_value_free(value: *mut GblnValue) {
    if !value.is_null() {
        unsafe {
            drop(Box::from_raw(value));
        }
    }
}

/// Serialize GBLN value to compact string
///
/// # Safety
/// - `value` must be a valid GblnValue pointer
/// - Caller must free the returned string with `gbln_string_free()`
///
/// # Returns
/// - Pointer to null-terminated C string on success
/// - NULL on error
#[no_mangle]
pub extern "C" fn gbln_to_string(value: *const GblnValue) -> *mut c_char {
    if value.is_null() {
        set_last_error("Null pointer".to_string(), None);
        return ptr::null_mut();
    }

    let value = unsafe { (*value).inner() };
    let result = to_string(value);

    match CString::new(result) {
        Ok(s) => s.into_raw(),
        Err(_) => {
            set_last_error("Failed to create C string".to_string(), None);
            ptr::null_mut()
        }
    }
}

/// Serialize GBLN value to formatted string
///
/// # Safety
/// - `value` must be a valid GblnValue pointer
/// - Caller must free the returned string with `gbln_string_free()`
///
/// # Returns
/// - Pointer to null-terminated C string on success
/// - NULL on error
#[no_mangle]
pub extern "C" fn gbln_to_string_pretty(value: *const GblnValue) -> *mut c_char {
    if value.is_null() {
        set_last_error("Null pointer".to_string(), None);
        return ptr::null_mut();
    }

    let value = unsafe { (*value).inner() };
    let result = to_string_pretty(value);

    match CString::new(result) {
        Ok(s) => s.into_raw(),
        Err(_) => {
            set_last_error("Failed to create C string".to_string(), None);
            ptr::null_mut()
        }
    }
}

/// Free a string returned by GBLN functions
///
/// # Safety
/// - `s` must be a valid pointer returned from GBLN functions or NULL
/// - Must not be called twice on the same pointer
#[no_mangle]
pub extern "C" fn gbln_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

/// Get last error message
///
/// Returns NULL if no error occurred.
/// The returned pointer is valid until the next error occurs.
/// Caller must free with `gbln_string_free()`.
#[no_mangle]
pub extern "C" fn gbln_last_error_message() -> *mut c_char {
    match get_last_error() {
        Some((msg, _)) => match CString::new(msg) {
            Ok(s) => s.into_raw(),
            Err(_) => ptr::null_mut(),
        },
        None => ptr::null_mut(),
    }
}

/// Get last error suggestion
///
/// Returns NULL if no suggestion is available.
/// The returned pointer is valid until the next error occurs.
/// Caller must free with `gbln_string_free()`.
#[no_mangle]
pub extern "C" fn gbln_last_error_suggestion() -> *mut c_char {
    match get_last_error() {
        Some((_, Some(suggestion))) => match CString::new(suggestion) {
            Ok(s) => s.into_raw(),
            Err(_) => ptr::null_mut(),
        },
        _ => ptr::null_mut(),
    }
}

/// Get field from object
///
/// # Safety
/// - `value` must be a valid GblnValue pointer
/// - `key` must be a valid null-terminated UTF-8 string
/// - Returns NULL if value is not an object or key not found
/// - Returned pointer is valid as long as the parent value is valid
#[no_mangle]
pub extern "C" fn gbln_object_get(value: *const GblnValue, key: *const c_char) -> *const GblnValue {
    if value.is_null() || key.is_null() {
        return ptr::null();
    }

    let key_str = unsafe {
        match CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null(),
        }
    };

    match unsafe { (*value).inner() } {
        Value::Object(map) => map
            .get(key_str)
            .map(|v| v as *const Value as *const GblnValue)
            .unwrap_or(ptr::null()),
        _ => ptr::null(),
    }
}

/// Get array length
///
/// # Safety
/// - `value` must be a valid GblnValue pointer
/// - Returns 0 if value is not an array
#[no_mangle]
pub extern "C" fn gbln_array_len(value: *const GblnValue) -> usize {
    if value.is_null() {
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::Array(arr) => arr.len(),
        _ => 0,
    }
}

/// Get array element by index
///
/// # Safety
/// - `value` must be a valid GblnValue pointer
/// - Returns NULL if value is not an array or index out of bounds
/// - Returned pointer is valid as long as the parent value is valid
#[no_mangle]
pub extern "C" fn gbln_array_get(value: *const GblnValue, index: usize) -> *const GblnValue {
    if value.is_null() {
        return ptr::null();
    }

    match unsafe { (*value).inner() } {
        Value::Array(arr) => arr
            .get(index)
            .map(|v| v as *const Value as *const GblnValue)
            .unwrap_or(ptr::null()),
        _ => ptr::null(),
    }
}
