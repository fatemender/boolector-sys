[![crates.io](https://img.shields.io/crates/v/boolector-sys.svg)](https://crates.io/crates/boolector-sys)

# boolector-sys

This Rust crate provides low-level bindings for the [Boolector] SMT solver,
version 3.0.0.  It has the following limitations:

* the `boolector` library must be available on the system, no attempt is made to
  build it from source;
* the `boolector` library must be a shared library, otherwise its dependencies
  on the SAT solvers and `btor2parser` will not be satisfied.

[Boolector]: https://boolector.github.io/

## Installation

First, compile `boolector` as a shared library and install it.  Then add this
crate to your `Cargo.toml`:

```toml
[dependencies]
boolector-sys = "0.3"
```

## License

This crate is licensed under the [MIT license].

[MIT license]: LICENSE
