// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

//! FFI wrapper for GblnConfig
//!
//! Provides C-compatible configuration management for GBLN I/O format.

use gbln::GblnConfig as RustConfig;

/// Opaque wrapper for GblnConfig
#[repr(C)]
pub struct GblnConfig {
    pub(crate) inner: RustConfig,
}

/// Create default I/O configuration
///
/// Returns configuration for production I/O format:
/// - mini_mode: true
/// - compress: true
/// - compression_level: 6
/// - indent: 2
/// - strip_comments: true
///
/// # Safety
/// Caller must free with `gbln_config_free()`
#[no_mangle]
pub extern "C" fn gbln_config_new_io() -> *mut GblnConfig {
    let config = Box::new(GblnConfig {
        inner: RustConfig::io_format(),
    });
    Box::into_raw(config)
}

/// Create default source configuration
///
/// Returns configuration for human-readable source:
/// - mini_mode: false
/// - compress: false
/// - compression_level: 6
/// - indent: 2
/// - strip_comments: false
///
/// # Safety
/// Caller must free with `gbln_config_free()`
#[no_mangle]
pub extern "C" fn gbln_config_new_source() -> *mut GblnConfig {
    let config = Box::new(GblnConfig {
        inner: RustConfig::development(),
    });
    Box::into_raw(config)
}

/// Create custom configuration
///
/// # Parameters
/// - mini_mode: Use MINI GBLN (no whitespace)
/// - compress: Enable XZ compression
/// - compression_level: XZ level (0-9, where 9 is maximum)
/// - indent: Indentation width (0 = no indent)
/// - strip_comments: Remove comments in output
///
/// # Safety
/// Caller must free with `gbln_config_free()`
#[no_mangle]
pub extern "C" fn gbln_config_new(
    mini_mode: bool,
    compress: bool,
    compression_level: u8,
    indent: usize,
    strip_comments: bool,
) -> *mut GblnConfig {
    let config = Box::new(GblnConfig {
        inner: RustConfig {
            mini_mode,
            compress,
            compression_level,
            indent,
            strip_comments,
        },
    });
    Box::into_raw(config)
}

/// Free configuration
///
/// # Safety
/// - config must be a valid pointer from gbln_config_new_*() or NULL
/// - Must not be called twice on the same pointer
#[no_mangle]
pub extern "C" fn gbln_config_free(config: *mut GblnConfig) {
    if !config.is_null() {
        unsafe {
            let _ = Box::from_raw(config);
        }
    }
}

// Getters

/// Get mini_mode setting
#[no_mangle]
pub extern "C" fn gbln_config_get_mini_mode(config: *const GblnConfig) -> bool {
    if config.is_null() {
        return false;
    }
    unsafe { (*config).inner.mini_mode }
}

/// Get compress setting
#[no_mangle]
pub extern "C" fn gbln_config_get_compress(config: *const GblnConfig) -> bool {
    if config.is_null() {
        return false;
    }
    unsafe { (*config).inner.compress }
}

/// Get compression_level setting
#[no_mangle]
pub extern "C" fn gbln_config_get_compression_level(config: *const GblnConfig) -> u8 {
    if config.is_null() {
        return 6;
    }
    unsafe { (*config).inner.compression_level }
}

/// Get indent setting
#[no_mangle]
pub extern "C" fn gbln_config_get_indent(config: *const GblnConfig) -> usize {
    if config.is_null() {
        return 2;
    }
    unsafe { (*config).inner.indent }
}

/// Get strip_comments setting
#[no_mangle]
pub extern "C" fn gbln_config_get_strip_comments(config: *const GblnConfig) -> bool {
    if config.is_null() {
        return false;
    }
    unsafe { (*config).inner.strip_comments }
}

// Setters

/// Set mini_mode setting
#[no_mangle]
pub extern "C" fn gbln_config_set_mini_mode(config: *mut GblnConfig, value: bool) {
    if !config.is_null() {
        unsafe {
            (*config).inner.mini_mode = value;
        }
    }
}

/// Set compress setting
#[no_mangle]
pub extern "C" fn gbln_config_set_compress(config: *mut GblnConfig, value: bool) {
    if !config.is_null() {
        unsafe {
            (*config).inner.compress = value;
        }
    }
}

/// Set compression_level setting (0-9)
#[no_mangle]
pub extern "C" fn gbln_config_set_compression_level(config: *mut GblnConfig, value: u8) {
    if !config.is_null() {
        unsafe {
            (*config).inner.compression_level = value;
        }
    }
}

/// Set indent setting
#[no_mangle]
pub extern "C" fn gbln_config_set_indent(config: *mut GblnConfig, value: usize) {
    if !config.is_null() {
        unsafe {
            (*config).inner.indent = value;
        }
    }
}

/// Set strip_comments setting
#[no_mangle]
pub extern "C" fn gbln_config_set_strip_comments(config: *mut GblnConfig, value: bool) {
    if !config.is_null() {
        unsafe {
            (*config).inner.strip_comments = value;
        }
    }
}
