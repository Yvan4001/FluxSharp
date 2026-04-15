; FluxSharp Core Library Include File
; Provides standard type definitions and built-in function declarations
;
; This file is automatically included by the compiler
; It defines the core types and functions available in FluxSharp
; ============================================================================
; Type Definitions
; ============================================================================
; Primitive Types (all 64-bit in current implementation)
; - int       : 64-bit signed integer
; - float     : 32-bit IEEE 754 floating point
; - double    : 64-bit IEEE 754 floating point
; - string    : null-terminated character array
; - bool      : 1 byte (0 = false, non-zero = true)
; - null      : null pointer (0)
; ============================================================================
; Built-in Functions
; ============================================================================
; I/O Functions
extern _fsh_print_int
extern _fsh_print_str
; Math Functions
extern _fsh_abs
extern _fsh_sqrt
extern _fsh_pow
extern _fsh_floor
extern _fsh_ceil
extern _fsh_round
extern _fsh_max
extern _fsh_min
; String Functions
extern _fsh_string_length
