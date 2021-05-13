# Vararg
![vararg tests status](https://github.com/Morgan-iv/rs-vararg/actions/workflows/rust.yml/badge.svg)

This crate provides `vararg` proc-macro, that can be applied to any
function which last arg is `Vec`, array or reference to slice, turning
it into variadic macro, similar to `println!` or `format!`

## Examples

```rust
use vararg::vararg;

#[vararg]
fn vararg_func<const L: usize>(a: &str, arr: [&str; L]) -> String {
    format!("\nfirst: {} \nlast: {} \n", a, arr.join(","))
}

fn main() {
    let s1 = "\n\
        first: first \n\
        last:  \n\
    ";
    let s2 = "\n\
        first: first \n\
        last: last \n\
    ";
    let s3 = "\n\
        first: first \n\
        last: last1,last2 \n\
    ";
    assert_eq!(s1, vararg_func!("first"));
    assert_eq!(s2, vararg_func!("first", "last"));
    assert_eq!(s3, vararg_func!("first", "last1", "last2"));
}
```

You can create vararg function without first required argument (unlike
vararg functions in C or nightly Rust). For example, let's create vararg
function to join String slices without separator:

```rust
use vararg::vararg;

#[vararg]
fn join_strs<const L: usize>(arr: [&str; L]) -> String {
    arr.join("")
}

fn main() {
    let s1 = "";
    let s2 = "ha";
    let s3 = "hahaha";
    assert_eq!(s1, join_strs!());
    assert_eq!(s2, join_strs!("ha"));
    assert_eq!(s3, join_strs!("ha", "ha", "ha"));
}
```

You also can change name of generated macro or type of last arg

```rust
use vararg::vararg;
use std::process::Command;

#[vararg(name = exec, type = slice)] // or type = array (default), or type = vec
fn exec_process(basename: &str, args: &[&str]) -> String {
    String::from_utf8(
        Command::new(basename)
            .args(args)
            .output()
            .expect("failed to execute echo")
            .stdout,
    )
    .expect("failed to parse hello")
}

fn main() {
    let (name, c) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
    assert_eq!("hello\n", exec!(name, c, "echo hello"));
}
```

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.
