#ifndef GBLN_H
#define GBLN_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * C-compatible error codes
 *
 * These match the Rust ErrorKind variants from gbln-rust
 */
typedef enum GblnErrorCode {
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
} GblnErrorCode;

/**
 * Opaque pointer to a GBLN value
 *
 * This is a C-compatible wrapper around Rust's Value type.
 * C code sees this as an opaque pointer and must use the provided
 * accessor functions to interact with it.
 *
 * Note: This struct is intentionally opaque to C. The internal
 * Value type is not exposed in the C header.
 */
typedef struct GblnValue GblnValue;

/**
 * Parse GBLN string into a value
 *
 * # Safety
 * - `input` must be a valid null-terminated UTF-8 string
 * - `out_value` must be a valid pointer to store the result
 * - Caller must free the returned value with `gbln_value_free()`
 *
 * # Returns
 * - `GBLN_OK` on success, with `out_value` set to the parsed value
 * - Error code on failure, with error details available via `gbln_last_error_message()`
 */
enum GblnErrorCode gbln_parse(const char *input, struct GblnValue **out_value);

/**
 * Free a GBLN value
 *
 * # Safety
 * - `value` must be a valid pointer returned from `gbln_parse()` or NULL
 * - Must not be called twice on the same pointer
 */
void gbln_value_free(struct GblnValue *value);

/**
 * Serialize GBLN value to compact string
 *
 * # Safety
 * - `value` must be a valid GblnValue pointer
 * - Caller must free the returned string with `gbln_string_free()`
 *
 * # Returns
 * - Pointer to null-terminated C string on success
 * - NULL on error
 */
char *gbln_to_string(const struct GblnValue *value);

/**
 * Serialize GBLN value to formatted string
 *
 * # Safety
 * - `value` must be a valid GblnValue pointer
 * - Caller must free the returned string with `gbln_string_free()`
 *
 * # Returns
 * - Pointer to null-terminated C string on success
 * - NULL on error
 */
char *gbln_to_string_pretty(const struct GblnValue *value);

/**
 * Free a string returned by GBLN functions
 *
 * # Safety
 * - `s` must be a valid pointer returned from GBLN functions or NULL
 * - Must not be called twice on the same pointer
 */
void gbln_string_free(char *s);

/**
 * Get last error message
 *
 * Returns NULL if no error occurred.
 * The returned pointer is valid until the next error occurs.
 * Caller must free with `gbln_string_free()`.
 */
char *gbln_last_error_message(void);

/**
 * Get last error suggestion
 *
 * Returns NULL if no suggestion is available.
 * The returned pointer is valid until the next error occurs.
 * Caller must free with `gbln_string_free()`.
 */
char *gbln_last_error_suggestion(void);

/**
 * Get field from object
 *
 * # Safety
 * - `value` must be a valid GblnValue pointer
 * - `key` must be a valid null-terminated UTF-8 string
 * - Returns NULL if value is not an object or key not found
 * - Returned pointer is valid as long as the parent value is valid
 */
const struct GblnValue *gbln_object_get(const struct GblnValue *value, const char *key);

/**
 * Get array length
 *
 * # Safety
 * - `value` must be a valid GblnValue pointer
 * - Returns 0 if value is not an array
 */
uintptr_t gbln_array_len(const struct GblnValue *value);

/**
 * Get array element by index
 *
 * # Safety
 * - `value` must be a valid GblnValue pointer
 * - Returns NULL if value is not an array or index out of bounds
 * - Returned pointer is valid as long as the parent value is valid
 */
const struct GblnValue *gbln_array_get(const struct GblnValue *value, uintptr_t index);

/**
 * Get i8 value
 *
 * # Safety
 * - `value` must be a valid GblnValue pointer
 * - `ok` will be set to true if value is i8, false otherwise
 */
int8_t gbln_value_as_i8(const struct GblnValue *value, bool *ok);

/**
 * Get i16 value
 */
int16_t gbln_value_as_i16(const struct GblnValue *value, bool *ok);

/**
 * Get i32 value
 */
int32_t gbln_value_as_i32(const struct GblnValue *value, bool *ok);

/**
 * Get i64 value
 */
int64_t gbln_value_as_i64(const struct GblnValue *value, bool *ok);

/**
 * Get u8 value
 */
uint8_t gbln_value_as_u8(const struct GblnValue *value, bool *ok);

/**
 * Get u16 value
 */
uint16_t gbln_value_as_u16(const struct GblnValue *value, bool *ok);

/**
 * Get u32 value
 */
uint32_t gbln_value_as_u32(const struct GblnValue *value, bool *ok);

/**
 * Get u64 value
 */
uint64_t gbln_value_as_u64(const struct GblnValue *value, bool *ok);

/**
 * Get f32 value
 */
float gbln_value_as_f32(const struct GblnValue *value, bool *ok);

/**
 * Get f64 value
 */
double gbln_value_as_f64(const struct GblnValue *value, bool *ok);

/**
 * Get string value
 *
 * # Safety
 * - Returns a pointer to a null-terminated C string
 * - Caller must free the returned string with `gbln_string_free()`
 * - Returns NULL if value is not a string
 */
char *gbln_value_as_string(const struct GblnValue *value, bool *ok);

/**
 * Get bool value
 */
bool gbln_value_as_bool(const struct GblnValue *value, bool *ok);

/**
 * Check if value is null
 */
bool gbln_value_is_null(const struct GblnValue *value);

#endif  /* GBLN_H */
