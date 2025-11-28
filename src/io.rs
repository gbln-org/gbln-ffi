// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

//! FFI I/O functions for GBLN I/O format
//!
//! Provides write_io and read_io functions for creating and reading
//! GBLN I/O format files (.io.gbln.xz)

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;

use crate::config::GblnConfig;
use crate::error::{set_last_error, GblnErrorCode};
use crate::types::GblnValue;
use gbln::{read_io as rust_read_io, write_io as rust_write_io};

/// Write GBLN value to I/O format file
///
/// This function serialises the value according to the configuration and writes
/// it to the specified file. The file extension and compression are determined
/// by the config settings.
///
/// # File Extensions
/// - `.io.gbln.xz`: MINI GBLN + XZ compression (compress=true)
/// - `.io.gbln`: MINI GBLN without compression (compress=false, mini_mode=true)
/// - `.gbln`: Pretty-printed source format (mini_mode=false)
///
/// # Parameters
/// - value: GBLN value to write
/// - path: File path (null-terminated string)
/// - config: I/O configuration (if NULL, uses default io_format())
///
/// # Returns
/// - GBLN_OK on success
/// - GBLN_ERROR_IO on file write failure
/// - GBLN_ERROR_NULL_POINTER if value or path is NULL
/// - Error details via gbln_last_error_message()
///
/// # Safety
/// - value must be a valid GblnValue pointer
/// - path must be a valid null-terminated UTF-8 string
/// - config may be NULL (uses default)
#[no_mangle]
pub extern "C" fn gbln_write_io(
    value: *const GblnValue,
    path: *const c_char,
    config: *const GblnConfig,
) -> GblnErrorCode {
    // Validate pointers
    if value.is_null() {
        set_last_error("Null value pointer".to_string(), None);
        return GblnErrorCode::ErrorNullPointer;
    }

    if path.is_null() {
        set_last_error("Null path pointer".to_string(), None);
        return GblnErrorCode::ErrorNullPointer;
    }

    // Convert C string to Rust string
    let path_str = unsafe {
        match CStr::from_ptr(path).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in path: {}", e), None);
                return GblnErrorCode::ErrorIo;
            }
        }
    };

    // Get config (or use default)
    let rust_config = if config.is_null() {
        gbln::GblnConfig::io_format()
    } else {
        unsafe { (*config).inner.clone() }
    };

    // Get value
    let rust_value = unsafe { (*value).inner() };

    // Write to file
    match rust_write_io(rust_value, Path::new(path_str), &rust_config) {
        Ok(()) => GblnErrorCode::Ok,
        Err(e) => {
            let suggestion = e.suggestion.clone();
            set_last_error(e.to_string(), suggestion);
            GblnErrorCode::ErrorIo
        }
    }
}

/// Read GBLN file from I/O format
///
/// This function reads a file and automatically detects if it's XZ compressed.
/// The content is then parsed into a GBLN value.
///
/// # Auto-Detection
/// The function checks for XZ magic bytes (FD 37 7A 58 5A 00) and automatically
/// decompresses if detected.
///
/// # Parameters
/// - path: File path (null-terminated string)
/// - out_value: Pointer to store the parsed value
///
/// # Returns
/// - GBLN_OK on success, with out_value set to parsed value
/// - GBLN_ERROR_IO on file read failure
/// - GBLN_ERROR_NULL_POINTER if path or out_value is NULL
/// - Parse errors on invalid GBLN content
/// - Error details via gbln_last_error_message()
///
/// # Safety
/// - path must be a valid null-terminated UTF-8 string
/// - out_value must be a valid pointer to store the result
/// - Caller must free returned value with gbln_value_free()
#[no_mangle]
pub extern "C" fn gbln_read_io(
    path: *const c_char,
    out_value: *mut *mut GblnValue,
) -> GblnErrorCode {
    // Validate pointers
    if path.is_null() || out_value.is_null() {
        set_last_error("Null pointer".to_string(), None);
        return GblnErrorCode::ErrorNullPointer;
    }

    // Convert C string to Rust string
    let path_str = unsafe {
        match CStr::from_ptr(path).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in path: {}", e), None);
                return GblnErrorCode::ErrorIo;
            }
        }
    };

    // Read from file
    match rust_read_io(Path::new(path_str)) {
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

            // Map error kind
            use gbln::ErrorKind;
            match e.kind {
                ErrorKind::IoError => GblnErrorCode::ErrorIo,
                ErrorKind::UnexpectedCharacter => GblnErrorCode::ErrorUnexpectedChar,
                ErrorKind::UnterminatedString => GblnErrorCode::ErrorUnterminatedString,
                ErrorKind::UnexpectedToken => GblnErrorCode::ErrorUnexpectedToken,
                ErrorKind::UnexpectedEof => GblnErrorCode::ErrorUnexpectedEof,
                ErrorKind::InvalidSyntax => GblnErrorCode::ErrorInvalidSyntax,
                ErrorKind::IntegerOutOfRange => GblnErrorCode::ErrorIntOutOfRange,
                ErrorKind::StringTooLong => GblnErrorCode::ErrorStringTooLong,
                ErrorKind::TypeMismatch => GblnErrorCode::ErrorTypeMismatch,
                ErrorKind::InvalidTypeHint => GblnErrorCode::ErrorInvalidTypeHint,
                ErrorKind::DuplicateKey => GblnErrorCode::ErrorDuplicateKey,
            }
        }
    }
}
