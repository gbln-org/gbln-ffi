/*
 * Test for Ticket #004A features:
 * - Optional type hints
 * - Typed arrays in objects
 * - Type inference
 */

#include "../include/gbln.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

void test_optional_types_single_values() {
    printf("Test 1: Optional type hints - single values\n");
    
    // name(Alice)
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse("name(Alice)", &value);
    assert(err == Ok);
    
    // Should infer as string
    bool ok;
    const struct GblnValue* obj_val = gbln_object_get(value, "name");
    assert(obj_val != NULL);
    char* name = gbln_value_as_string(obj_val, &ok);
    assert(ok == true);
    assert(strcmp(name, "Alice") == 0);
    gbln_string_free(name);
    
    gbln_value_free(value);
    printf("  ✅ PASSED\n\n");
}

void test_optional_types_in_object() {
    printf("Test 2: Optional type hints - object with multiple fields\n");
    
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse("{name(Alice)age(25)active(true)score(98.5)}", &value);
    assert(err == Ok);
    
    bool ok;
    
    // Check name (string)
    const struct GblnValue* name_val = gbln_object_get(value, "name");
    char* name = gbln_value_as_string(name_val, &ok);
    assert(ok == true);
    assert(strcmp(name, "Alice") == 0);
    gbln_string_free(name);
    
    // Check age (integer)
    const struct GblnValue* age_val = gbln_object_get(value, "age");
    int64_t age = gbln_value_as_i64(age_val, &ok);
    assert(ok == true);
    assert(age == 25);
    
    // Check active (boolean)
    const struct GblnValue* active_val = gbln_object_get(value, "active");
    bool active = gbln_value_as_bool(active_val, &ok);
    assert(ok == true);
    assert(active == true);
    
    // Check score (float)
    const struct GblnValue* score_val = gbln_object_get(value, "score");
    double score = gbln_value_as_f64(score_val, &ok);
    assert(ok == true);
    assert(score > 98.4 && score < 98.6);
    
    gbln_value_free(value);
    printf("  ✅ PASSED\n\n");
}

void test_untyped_arrays() {
    printf("Test 3: Untyped arrays with type inference\n");
    
    // Integer array
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse("numbers[1 2 3 4 5]", &value);
    assert(err == Ok);
    
    const struct GblnValue* arr = gbln_object_get(value, "numbers");
    assert(arr != NULL);
    size_t len = gbln_array_len(arr);
    assert(len == 5);
    
    bool ok;
    const struct GblnValue* first = gbln_array_get(arr, 0);
    int64_t val = gbln_value_as_i64(first, &ok);
    assert(ok == true);
    assert(val == 1);
    
    gbln_value_free(value);
    
    // String array
    value = NULL;
    err = gbln_parse("tags[rust python golang]", &value);
    assert(err == Ok);
    
    arr = gbln_object_get(value, "tags");
    len = gbln_array_len(arr);
    assert(len == 3);
    
    first = gbln_array_get(arr, 0);
    char* str = gbln_value_as_string(first, &ok);
    assert(ok == true);
    assert(strcmp(str, "rust") == 0);
    gbln_string_free(str);
    
    gbln_value_free(value);
    printf("  ✅ PASSED\n\n");
}

void test_typed_arrays_in_objects() {
    printf("Test 4: Typed arrays in objects (Ticket #004A)\n");
    
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse("{tags<s16>[rust python golang]}", &value);
    assert(err == Ok);
    
    const struct GblnValue* tags = gbln_object_get(value, "tags");
    assert(tags != NULL);
    
    size_t len = gbln_array_len(tags);
    assert(len == 3);
    
    bool ok;
    const struct GblnValue* first = gbln_array_get(tags, 0);
    char* tag = gbln_value_as_string(first, &ok);
    assert(ok == true);
    assert(strcmp(tag, "rust") == 0);
    gbln_string_free(tag);
    
    const struct GblnValue* second = gbln_array_get(tags, 1);
    tag = gbln_value_as_string(second, &ok);
    assert(ok == true);
    assert(strcmp(tag, "python") == 0);
    gbln_string_free(tag);
    
    gbln_value_free(value);
    printf("  ✅ PASSED\n\n");
}

void test_typed_arrays_validation() {
    printf("Test 5: Typed arrays with validation\n");
    
    // Valid typed array
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse("{scores<i32>[98 87 92]}", &value);
    assert(err == Ok);
    
    const struct GblnValue* scores = gbln_object_get(value, "scores");
    size_t len = gbln_array_len(scores);
    assert(len == 3);
    
    bool ok;
    const struct GblnValue* first = gbln_array_get(scores, 0);
    int32_t score = gbln_value_as_i32(first, &ok);
    assert(ok == true);
    assert(score == 98);
    
    gbln_value_free(value);
    
    // Invalid: value out of range
    value = NULL;
    err = gbln_parse("{ages<i8>[25 300]}", &value);
    assert(err == ErrorTypeMismatch);  // 300 out of range for i8
    
    char* error_msg = gbln_last_error_message();
    assert(error_msg != NULL);
    printf("  Expected error: %s\n", error_msg);
    gbln_string_free(error_msg);
    
    printf("  ✅ PASSED\n\n");
}

void test_mixed_typed_and_untyped() {
    printf("Test 6: Mixed typed and untyped fields\n");
    
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse("{id<u32>(12345)name(Alice)age<i8>(25)tags[rust python]}", &value);
    assert(err == Ok);
    
    bool ok;
    
    // Typed field
    const struct GblnValue* id_val = gbln_object_get(value, "id");
    uint32_t id = gbln_value_as_u32(id_val, &ok);
    assert(ok == true);
    assert(id == 12345);
    
    // Untyped field (inferred as string)
    const struct GblnValue* name_val = gbln_object_get(value, "name");
    char* name = gbln_value_as_string(name_val, &ok);
    assert(ok == true);
    assert(strcmp(name, "Alice") == 0);
    gbln_string_free(name);
    
    // Typed field
    const struct GblnValue* age_val = gbln_object_get(value, "age");
    int8_t age = gbln_value_as_i8(age_val, &ok);
    assert(ok == true);
    assert(age == 25);
    
    // Untyped array
    const struct GblnValue* tags = gbln_object_get(value, "tags");
    size_t len = gbln_array_len(tags);
    assert(len == 2);
    
    gbln_value_free(value);
    printf("  ✅ PASSED\n\n");
}

void test_hyphenated_identifiers() {
    printf("Test 7: Hyphenated identifiers in typed arrays\n");
    
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse("{tags<s16>[rust-lang python-dev golang-beta]}", &value);
    assert(err == Ok);
    
    const struct GblnValue* tags = gbln_object_get(value, "tags");
    size_t len = gbln_array_len(tags);
    assert(len == 3);
    
    bool ok;
    const struct GblnValue* first = gbln_array_get(tags, 0);
    char* tag = gbln_value_as_string(first, &ok);
    assert(ok == true);
    assert(strcmp(tag, "rust-lang") == 0);
    gbln_string_free(tag);
    
    gbln_value_free(value);
    printf("  ✅ PASSED\n\n");
}

void test_negative_numbers_in_arrays() {
    printf("Test 8: Negative numbers in untyped arrays\n");
    
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse("temps[-15 -5 0 5 15]", &value);
    assert(err == Ok);
    
    const struct GblnValue* temps = gbln_object_get(value, "temps");
    size_t len = gbln_array_len(temps);
    assert(len == 5);
    
    bool ok;
    const struct GblnValue* first = gbln_array_get(temps, 0);
    int64_t temp = gbln_value_as_i64(first, &ok);
    assert(ok == true);
    assert(temp == -15);
    
    gbln_value_free(value);
    printf("  ✅ PASSED\n\n");
}

void test_floating_point_in_arrays() {
    printf("Test 9: Floating-point numbers in untyped arrays\n");
    
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse("prices[19.99 29.99 9.99]", &value);
    assert(err == Ok);
    
    const struct GblnValue* prices = gbln_object_get(value, "prices");
    size_t len = gbln_array_len(prices);
    assert(len == 3);
    
    bool ok;
    const struct GblnValue* first = gbln_array_get(prices, 0);
    double price = gbln_value_as_f64(first, &ok);
    assert(ok == true);
    assert(price > 19.98 && price < 20.0);
    
    gbln_value_free(value);
    printf("  ✅ PASSED\n\n");
}

void test_nested_with_typed_arrays() {
    printf("Test 10: Nested objects with typed arrays\n");
    
    const char* input = "{"
        "user{"
            "name<s32>(Alice)"
            "tags<s16>[developer rust-fan]"
            "scores<i16>[98 87 92]"
        "}"
    "}";
    
    struct GblnValue* value = NULL;
    enum GblnErrorCode err = gbln_parse(input, &value);
    assert(err == Ok);
    
    const struct GblnValue* user = gbln_object_get(value, "user");
    assert(user != NULL);
    
    // Check tags array
    const struct GblnValue* tags = gbln_object_get(user, "tags");
    size_t len = gbln_array_len(tags);
    assert(len == 2);
    
    bool ok;
    const struct GblnValue* tag1 = gbln_array_get(tags, 0);
    char* tag_str = gbln_value_as_string(tag1, &ok);
    assert(ok == true);
    assert(strcmp(tag_str, "developer") == 0);
    gbln_string_free(tag_str);
    
    const struct GblnValue* tag2 = gbln_array_get(tags, 1);
    tag_str = gbln_value_as_string(tag2, &ok);
    assert(ok == true);
    assert(strcmp(tag_str, "rust-fan") == 0);
    gbln_string_free(tag_str);
    
    // Check scores array
    const struct GblnValue* scores = gbln_object_get(user, "scores");
    len = gbln_array_len(scores);
    assert(len == 3);
    
    const struct GblnValue* score1 = gbln_array_get(scores, 0);
    int16_t score = gbln_value_as_i16(score1, &ok);
    assert(ok == true);
    assert(score == 98);
    
    gbln_value_free(value);
    printf("  ✅ PASSED\n\n");
}

int main() {
    printf("\n===========================================\n");
    printf("GBLN FFI Tests for Ticket #004A Features\n");
    printf("===========================================\n\n");
    
    test_optional_types_single_values();
    test_optional_types_in_object();
    test_untyped_arrays();
    test_typed_arrays_in_objects();
    test_typed_arrays_validation();
    test_mixed_typed_and_untyped();
    test_hyphenated_identifiers();
    test_negative_numbers_in_arrays();
    test_floating_point_in_arrays();
    test_nested_with_typed_arrays();
    
    printf("===========================================\n");
    printf("✅ All 10 tests PASSED!\n");
    printf("===========================================\n\n");
    
    return 0;
}
