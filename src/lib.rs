//! A macro to compare structs, field by field.
//!
//! This crate provides the `compare_structs!` macro, which is useful for writing
//! `assert!`-style tests on structs. It provides more detailed output than a
//! standard `assert_eq!` on two struct instances.
//!
//! # Basic Usage
//!
//! To compare specific fields between two structs, provide the struct expressions
//! and the identifiers of the fields to compare.
//!
//! ```edition2024
//! use cmp::compare_structs;
//! # struct A { a: i32, b: &'static str }
//! # struct B { a: i32, b: &'static str }
//! let struct_a = A { a: 1, b: "hello" };
//! let struct_b = B { a: 1, b: "world" };
//!
//! // This will pass, as we only compare the `a` field.
//! compare_structs!(struct_a, struct_b, a);
//! ```
//!
//! # `serde` feature
//!
//! This crate has an optional `serde` feature that allows comparing all fields
//! of a struct without specifying them. To use it, enable the feature in your
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! cmp = { version = "1.0.0", features = ["serde"] }
//! ```
//!
//! With the `serde` feature enabled, you can call `compare_structs!` with just
//! two arguments. The structs must derive `serde::Serialize`.
//!
//! ```edition2024
//! # #[cfg(feature = "serde")]
//! # {
//! use cmp::compare_structs;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct MyStruct {
//!     field1: i32,
//!     field2: String,
//! }
//!
//! let a = MyStruct { field1: 1, field2: "test".to_string() };
//! let b = MyStruct { field1: 1, field2: "test".to_string() };
//!
//! // Compares all fields
//! compare_structs!(a, b);
//! # }
//! ```
//!
//! If there are missing fields in one of the expressions when using the `serde`
//! feature, the macro will panic with a clear error message indicating which
//! field is missing from which struct.

/// Macro which is mostly useful when writing `assert!` tests on structs.
///
/// ```edition2024
/// use cmp::compare_structs;
/// # struct A<'a> {
/// #     a: i32,
/// #     b: &'a str,
/// #     c: [(f64, f32); 2],
/// # }
/// # struct B<'a> {
/// #     a: i32,
/// #     b: &'a str,
/// #     c: [(f64, f32); 2],
/// # }
/// let struct_a = A {
///     a: 10,
///     b: "str",
///     c: [(1.0, 1.0), (2.0, 2.0)],
/// };
/// let struct_b = B {
///     a: 10,
///     b: "diff str",
///     c: [(1.0, 1.0), (2.0, 2.0)],
/// };
/// compare_structs!(struct_a, struct_b, a, c);
/// ```
///
/// Output singles-out fields in the struct which do not match:
///
/// ```bash
/// thread 'tests::compare_different_structs' panicked at src/lib.rs:135:9:
/// c: [
///     (
///         1.0,
///         1.0,
///     ),
///     (
///         2.0,
///         3.0,
///     ),
/// ] != [
///     (
///         1.0,
///         1.0,
///     ),
///     (
///         2.0,
///         2.0,
///     ),
/// ]
///
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
/// ```
///
/// The main motivation behind this macro is for structs with many fields, where `assert_eq!(struct_a, struct_b)`'s output is difficult to read.
///
/// /// # Panics
///
/// Panics if any of the fields do not have partial equality.
#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! compare_structs {
    ($expected:expr, $actual:expr, $($field:ident),+) => {{
        let mut diffs = String::new();
        $(
            if $expected.$field != $actual.$field {
                diffs.push_str(&format!(
                    "{}: {:#?} != {:#?}\n",
                    stringify!($field),
                    $expected.$field,
                    $actual.$field
                ));
            }
        )+

        assert!(diffs.is_empty(), "{diffs}");
    }};
}

#[cfg(feature = "serde")]
#[macro_export]
macro_rules! compare_structs {
    ($expected:expr, $actual:expr) => {{
        let expected_val =
            serde_json::to_value(&$expected).expect("Could not serialize expected value");
        let actual_val = serde_json::to_value(&$actual).expect("Could not serialize actual value");

        if expected_val != actual_val {
            let expected_map = expected_val
                .as_object()
                .expect("Expected value is not an object");
            let actual_map = actual_val
                .as_object()
                .expect("Actual value is not an object");
            let mut diffs = String::new();

            for (key, expected_field_val) in expected_map {
                match actual_map.get(key) {
                    Some(actual_field_val) => {
                        if expected_field_val != actual_field_val {
                            diffs.push_str(&format!(
                                "{}: {:#?} != {:#?}\n",
                                key, expected_field_val, actual_field_val
                            ));
                        }
                    }
                    None => {
                        diffs.push_str(&format!(
                            "{}: field missing from actual: {:#?}\n",
                            key, expected_field_val
                        ));
                    }
                }
            }

            for (key, actual_field_val) in actual_map {
                if !expected_map.contains_key(key) {
                    diffs.push_str(&format!(
                        "{}: field missing from expected: {:#?}\n",
                        key, actual_field_val
                    ));
                }
            }

            assert!(diffs.is_empty(), "{diffs}");
        }
    }};
    ($expected:expr, $actual:expr, $($field:ident),+) => {{
        let mut diffs = String::new();
        $(
            if $expected.$field != $actual.$field {
                diffs.push_str(&format!(
                    "{}: {:#?} != {:#?}\n",
                    stringify!($field),
                    $expected.$field,
                    $actual.$field
                ));
            }
        )+

        assert!(diffs.is_empty(), "{diffs}");
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "serde")]
    use serde::Serialize;

    #[cfg_attr(feature = "serde", derive(Serialize))]
    struct A<'a> {
        a: i32,
        b: &'a str,
        c: [(f64, f32); 2],
    }

    #[cfg_attr(feature = "serde", derive(Serialize))]
    struct B<'a> {
        a: i32,
        b: &'a str,
        c: [(f64, f32); 2],
    }

    static STRUCT_A: A = A {
        a: 10,
        b: "str",
        c: [(1.0, 1.0), (2.0, 2.0)],
    };

    static STRUCT_B: B = B {
        a: 10,
        b: "str",
        c: [(1.0, 1.0), (2.0, 2.0)],
    };

    #[test]
    #[cfg(feature = "serde")]
    fn compare_all_fields_no_args() {
        let struct_a = A {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.0, 2.0)],
        };

        let struct_b = B {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.0, 2.0)],
        };

        compare_structs!(struct_a, struct_b);
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "serde")]
    fn compare_all_fields_no_args_panic() {
        let struct_a = A {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.0, 2.0)],
        };

        let struct_b = B {
            a: 10,
            b: "different",
            c: [(1.0, 1.0), (2.0, 2.0)],
        };

        compare_structs!(struct_a, struct_b);
    }

    #[test]
    fn compare_all_fields() {
        let struct_a = A {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.0, 2.0)],
        };

        let struct_b = B {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.0, 2.0)],
        };

        compare_structs!(STRUCT_A, STRUCT_B, a, b, c);
        compare_structs!(struct_a, struct_b, a, b, c);
    }

    #[test]
    fn compare_some_fields() {
        let struct_a = A {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.0, 2.0)],
        };
        let struct_b = B {
            a: 10,
            b: "diff str",
            c: [(1.0, 1.0), (2.0, 2.0)],
        };

        compare_structs!(struct_a, struct_b, a, c);
    }

    #[test]
    fn compare_same_struct() {
        let struct_a = A {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.1, 2.0)],
        };

        let struct_a_again = A {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.1, 2.0)],
        };

        compare_structs!(struct_a, struct_a_again, a, b, c);
    }

    #[test]
    #[should_panic]
    fn compare_different_structs() {
        let struct_a = A {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.0, 3.0)],
        };

        let struct_b = B {
            a: 10,
            b: "str",
            c: [(1.0, 1.0), (2.0, 2.0)],
        };

        compare_structs!(struct_a, struct_b, a, b, c);
    }
}
