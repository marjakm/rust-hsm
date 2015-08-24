[![Build Status](https://travis-ci.org/marjakm/rust-hsm.svg?branch=master)](https://travis-ci.org/marjakm/rust-hsm)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/marjakm/rust-hsm/master/LICENSE)

# rust-hsm
Rust library for building hierarchical state machines.

## Usage
Put this in your `Cargo.toml`:

```toml
[dependencies.hsm]
git = "https://github.com/marjakm/rust-hsm.git"
```

And this in your crate root:

```rust
#[macro_use]
extern crate hsm;
```
