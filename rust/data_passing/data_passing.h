#pragma once

#include <stdint.h>

// -----------------------------------------------------------------------
// from primitives.rs
// -----------------------------------------------------------------------
int32_t return_int32(void);
uint16_t triple_a_uint16(uint16_t x);
float return_float(void);
double average_two_doubles(double x, double y);
size_t sum_sizes(size_t x, size_t y);

// -----------------------------------------------------------------------
// from strings.rs
// -----------------------------------------------------------------------
void c_string_to_rust(const char *null_terminated_string);
void utf8_bytes_to_rust(const uint8_t *bytes, size_t len);

struct RustByteSlice {
    const uint8_t *bytes;
    size_t len;
};

struct RustByteSlice get_string_from_rust();
