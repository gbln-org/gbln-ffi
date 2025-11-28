// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

use crate::types::GblnValue;
use gbln::Value;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

/// Get i8 value
///
/// # Safety
/// - `value` must be a valid GblnValue pointer
/// - `ok` will be set to true if value is i8, false otherwise
#[no_mangle]
pub extern "C" fn gbln_value_as_i8(value: *const GblnValue, ok: *mut bool) -> i8 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::I8(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0
        }
    }
}

/// Get i16 value
#[no_mangle]
pub extern "C" fn gbln_value_as_i16(value: *const GblnValue, ok: *mut bool) -> i16 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::I16(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0
        }
    }
}

/// Get i32 value
#[no_mangle]
pub extern "C" fn gbln_value_as_i32(value: *const GblnValue, ok: *mut bool) -> i32 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::I32(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0
        }
    }
}

/// Get i64 value
#[no_mangle]
pub extern "C" fn gbln_value_as_i64(value: *const GblnValue, ok: *mut bool) -> i64 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::I64(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0
        }
    }
}

/// Get u8 value
#[no_mangle]
pub extern "C" fn gbln_value_as_u8(value: *const GblnValue, ok: *mut bool) -> u8 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::U8(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0
        }
    }
}

/// Get u16 value
#[no_mangle]
pub extern "C" fn gbln_value_as_u16(value: *const GblnValue, ok: *mut bool) -> u16 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::U16(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0
        }
    }
}

/// Get u32 value
#[no_mangle]
pub extern "C" fn gbln_value_as_u32(value: *const GblnValue, ok: *mut bool) -> u32 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::U32(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0
        }
    }
}

/// Get u64 value
#[no_mangle]
pub extern "C" fn gbln_value_as_u64(value: *const GblnValue, ok: *mut bool) -> u64 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0;
    }

    match unsafe { (*value).inner() } {
        Value::U64(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0
        }
    }
}

/// Get f32 value
#[no_mangle]
pub extern "C" fn gbln_value_as_f32(value: *const GblnValue, ok: *mut bool) -> f32 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0.0;
    }

    match unsafe { (*value).inner() } {
        Value::F32(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0.0
        }
    }
}

/// Get f64 value
#[no_mangle]
pub extern "C" fn gbln_value_as_f64(value: *const GblnValue, ok: *mut bool) -> f64 {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return 0.0;
    }

    match unsafe { (*value).inner() } {
        Value::F64(n) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *n
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            0.0
        }
    }
}

/// Get string value
///
/// # Safety
/// - Returns a pointer to a null-terminated C string
/// - Caller must free the returned string with `gbln_string_free()`
/// - Returns NULL if value is not a string
#[no_mangle]
pub extern "C" fn gbln_value_as_string(value: *const GblnValue, ok: *mut bool) -> *mut c_char {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return ptr::null_mut();
    }

    match unsafe { (*value).inner() } {
        Value::Str(s) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            match CString::new(s.as_str()) {
                Ok(cs) => cs.into_raw(),
                Err(_) => {
                    if !ok.is_null() {
                        unsafe {
                            *ok = false;
                        }
                    }
                    ptr::null_mut()
                }
            }
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            ptr::null_mut()
        }
    }
}

/// Get bool value
#[no_mangle]
pub extern "C" fn gbln_value_as_bool(value: *const GblnValue, ok: *mut bool) -> bool {
    if value.is_null() {
        if !ok.is_null() {
            unsafe {
                *ok = false;
            }
        }
        return false;
    }

    match unsafe { (*value).inner() } {
        Value::Bool(b) => {
            if !ok.is_null() {
                unsafe {
                    *ok = true;
                }
            }
            *b
        }
        _ => {
            if !ok.is_null() {
                unsafe {
                    *ok = false;
                }
            }
            false
        }
    }
}

/// Check if value is null
#[no_mangle]
pub extern "C" fn gbln_value_is_null(value: *const GblnValue) -> bool {
    if value.is_null() {
        return true;
    }

    matches!(unsafe { (*value).inner() }, Value::Null)
}
