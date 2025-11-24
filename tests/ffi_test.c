#include "../include/gbln.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

void test_parse_simple() {
    const char* input = "{id<u32>(12345)name<s32>(Alice)}";
    struct GblnValue* value = NULL;

    enum GblnErrorCode err = gbln_parse(input, &value);
    assert(err == Ok);

    const struct GblnValue* id = gbln_object_get(value, "id");

    bool ok;
    uint32_t id_val = gbln_value_as_u32(id, &ok);
    assert(ok == true);
    assert(id_val == 12345);

    gbln_value_free(value);
    printf("test_parse_simple: PASSED\n");
}

void test_all_integer_types() {
    const char* input = "{i8<i8>(-128)i16<i16>(-32768)i32<i32>(-2147483648)"
                        "i64<i64>(-9223372036854775808)u8<u8>(255)u16<u16>(65535)"
                        "u32<u32>(4294967295)u64<u64>(18446744073709551615)}";
    struct GblnValue* value = NULL;

    enum GblnErrorCode err = gbln_parse(input, &value);
    assert(err == Ok);

    bool ok;

    // Test i8
    const struct GblnValue* i8_val = gbln_object_get(value, "i8");
    assert(gbln_value_as_i8(i8_val, &ok) == -128);
    assert(ok == true);

    // Test i16
    const struct GblnValue* i16_val = gbln_object_get(value, "i16");
    assert(gbln_value_as_i16(i16_val, &ok) == -32768);
    assert(ok == true);

    // Test i32
    const struct GblnValue* i32_val = gbln_object_get(value, "i32");
    assert(gbln_value_as_i32(i32_val, &ok) == -2147483648);
    assert(ok == true);

    // Test u8
    const struct GblnValue* u8_val = gbln_object_get(value, "u8");
    assert(gbln_value_as_u8(u8_val, &ok) == 255);
    assert(ok == true);

    // Test u16
    const struct GblnValue* u16_val = gbln_object_get(value, "u16");
    assert(gbln_value_as_u16(u16_val, &ok) == 65535);
    assert(ok == true);

    // Test u32
    const struct GblnValue* u32_val = gbln_object_get(value, "u32");
    assert(gbln_value_as_u32(u32_val, &ok) == 4294967295U);
    assert(ok == true);

    gbln_value_free(value);
    printf("test_all_integer_types: PASSED\n");
}

void test_float_types() {
    const char* input = "{f32<f32>(3.14159)f64<f64>(2.718281828459045)}";
    struct GblnValue* value = NULL;

    enum GblnErrorCode err = gbln_parse(input, &value);
    assert(err == Ok);

    bool ok;

    const struct GblnValue* f32_val = gbln_object_get(value, "f32");
    float f32_result = gbln_value_as_f32(f32_val, &ok);
    assert(ok == true);
    assert(f32_result > 3.14 && f32_result < 3.15);

    const struct GblnValue* f64_val = gbln_object_get(value, "f64");
    double f64_result = gbln_value_as_f64(f64_val, &ok);
    assert(ok == true);
    assert(f64_result > 2.71 && f64_result < 2.72);

    gbln_value_free(value);
    printf("test_float_types: PASSED\n");
}

void test_string_and_bool() {
    const char* input = "{name<s32>(Alice Johnson)active<b>(t)}";
    struct GblnValue* value = NULL;

    enum GblnErrorCode err = gbln_parse(input, &value);
    assert(err == Ok);

    bool ok;

    const struct GblnValue* name_val = gbln_object_get(value, "name");
    char* name = gbln_value_as_string(name_val, &ok);
    assert(ok == true);
    assert(strcmp(name, "Alice Johnson") == 0);
    gbln_string_free(name);

    const struct GblnValue* active_val = gbln_object_get(value, "active");
    bool active = gbln_value_as_bool(active_val, &ok);
    assert(ok == true);
    assert(active == true);

    gbln_value_free(value);
    printf("test_string_and_bool: PASSED\n");
}

void test_null_value() {
    const char* input = "{optional<n>()}";
    struct GblnValue* value = NULL;

    enum GblnErrorCode err = gbln_parse(input, &value);
    assert(err == Ok);

    const struct GblnValue* optional_val = gbln_object_get(value, "optional");
    assert(gbln_value_is_null(optional_val) == true);

    gbln_value_free(value);
    printf("test_null_value: PASSED\n");
}

void test_array() {
    // Note: Typed arrays in objects not yet supported in parser
    // Using array of objects instead
    const char* input = "{items[{name<s8>(rust)}{name<s8>(python)}{name<s8>(golang)}]}";
    struct GblnValue* value = NULL;

    enum GblnErrorCode err = gbln_parse(input, &value);
    assert(err == Ok);

    const struct GblnValue* items = gbln_object_get(value, "items");
    assert(items != NULL);
    
    size_t len = gbln_array_len(items);
    assert(len == 3);

    const struct GblnValue* first = gbln_array_get(items, 0);
    assert(first != NULL);
    
    const struct GblnValue* name = gbln_object_get(first, "name");
    bool ok;
    char* first_str = gbln_value_as_string(name, &ok);
    assert(ok == true);
    assert(strcmp(first_str, "rust") == 0);
    gbln_string_free(first_str);

    gbln_value_free(value);
    printf("test_array: PASSED\n");
}

void test_serialization() {
    const char* input = "{id<u32>(12345)name<s32>(Alice)}";
    struct GblnValue* value = NULL;

    enum GblnErrorCode err = gbln_parse(input, &value);
    assert(err == Ok);

    // Compact serialization
    char* compact = gbln_to_string(value);
    assert(compact != NULL);
    printf("Compact: %s\n", compact);
    gbln_string_free(compact);

    // Pretty serialization
    char* pretty = gbln_to_string_pretty(value);
    assert(pretty != NULL);
    printf("Pretty:\n%s\n", pretty);
    gbln_string_free(pretty);

    gbln_value_free(value);
    printf("test_serialization: PASSED\n");
}

void test_error_handling() {
    const char* input = "{age<i8>(999)}";  // Out of range
    struct GblnValue* value = NULL;

    enum GblnErrorCode err = gbln_parse(input, &value);
    assert(err == ErrorTypeMismatch);

    char* error_msg = gbln_last_error_message();
    assert(error_msg != NULL);
    printf("Error message: %s\n", error_msg);
    gbln_string_free(error_msg);

    char* suggestion = gbln_last_error_suggestion();
    if (suggestion != NULL) {
        printf("Suggestion: %s\n", suggestion);
        gbln_string_free(suggestion);
    }

    printf("test_error_handling: PASSED\n");
}

int main() {
    printf("Running GBLN FFI Tests...\n\n");

    test_parse_simple();
    test_all_integer_types();
    test_float_types();
    test_string_and_bool();
    test_null_value();
    test_array();
    test_serialization();
    test_error_handling();

    printf("\n✅ All tests PASSED!\n");
    return 0;
}
