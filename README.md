[![crates.io](https://img.shields.io/crates/v/boolector-sys.svg)](https://crates.io/crates/boolector-sys)

# boolector-sys

This Rust crate provides low-level bindings for the [Boolector] SMT solver.  It
has the following limitations:

* the `boolector` library must be available on the system, no attempt is made to
  build it from source;
* the `boolector` library must be a shared library, otherwise its dependencies
  on the SAT solvers and `btor2parser` will not be satisfied.

[Boolector]: https://boolector.github.io/

## Installation

First, compile and install the `boolector` shared library.  Then add this crate
to your `Cargo.toml`:

```toml
[dependencies]
boolector-sys = "0.1"
```

## License

This crate is licensed under the [MIT license].

[MIT license]: LICENSE
