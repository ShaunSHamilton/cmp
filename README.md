# cmp

`cmp` is a Rust crate that provides a macro for comparing fields in structs, which is particularly useful when writing `assert!` tests.

## Usage

To use this crate, add the following to your `Cargo.toml`:

```toml
[dependencies]
cmp = "0.1.0"
```

Then, in your Rust file:

```rust
use cmp::compare_structs;
```

## `compare_structs!`

The `compare_structs!` macro compares specified fields of two structs. If the fields do not match, the macro will panic and output the fields that do not match.

### Example

```rust
use cmp::compare_structs;

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
```

In this example, the `compare_structs!` macro compares the `a` and `c` fields of `struct_a` and `struct_b`. If they do not match, the macro will panic and output the fields that do not match.

## Output

The output of the `compare_structs!` macro singles out the fields in the structs that do not match. For example:

```bash
thread 'tests::compare_different_structs' panicked at src/lib.rs:135:9:
c: [
    (
        1.0,
        1.0,
    ),
    (
        2.0,
        3.0,
    ),
] != [
    (
        1.0,
        1.0,
    ),
    (
        2.0,
        2.0,
    ),
]

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In this output, the `c` field of the two structs do not match, and the macro outputs the differing values.
