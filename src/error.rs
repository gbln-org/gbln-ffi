use gbln::ErrorKind;
use std::cell::RefCell;

/// C-compatible error codes
///
/// These match the Rust ErrorKind variants from gbln-rust
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GblnErrorCode {
    Ok = 0,
    ErrorUnexpectedChar = 1,
    ErrorUnterminatedString = 2,
    ErrorUnexpectedToken = 3,
    ErrorUnexpectedEof = 4,
    ErrorInvalidSyntax = 5,
    ErrorIntOutOfRange = 6,
    ErrorStringTooLong = 7,
    ErrorTypeMismatch = 8,
    ErrorInvalidTypeHint = 9,
    ErrorDuplicateKey = 10,
    ErrorNullPointer = 11,
    ErrorIo = 12,
}

// Thread-local error storage
//
// Stores the last error message and optional suggestion.
// This allows C code to retrieve error details after a failed operation.
thread_local! {
    static LAST_ERROR: RefCell<Option<(String, Option<String>)>> = RefCell::new(None);
}

/// Set the last error message and optional suggestion
pub fn set_last_error(msg: String, suggestion: Option<String>) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = Some((msg, suggestion));
    });
}

/// Get the last error message and suggestion
pub fn get_last_error() -> Option<(String, Option<String>)> {
    LAST_ERROR.with(|e| e.borrow().clone())
}

/// Map Rust ErrorKind to C error code
pub fn map_error_kind(kind: &ErrorKind) -> GblnErrorCode {
    match kind {
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
        ErrorKind::IoError => GblnErrorCode::ErrorIo,
    }
}
