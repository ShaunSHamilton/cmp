/// Macro which is mostly useful when writing `assert!` tests on structs.
///
/// ```edition2021
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

#[cfg(test)]
mod tests {
    use super::*;

    struct A<'a> {
        a: i32,
        b: &'a str,
        c: [(f64, f32); 2],
    }

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
