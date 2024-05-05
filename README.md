<div align="center">
<br/>
<div align="left">
<br/>
</div>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io (latest)](https://img.shields.io/crates/v/padder)](https://crates.io/crates/padder)
[![codecov](https://codecov.io/gh/firelink-data/padder/graph/badge.svg?token=64QB0J4QMV)](https://codecov.io/gh/firelink-data/padder)
[![CI](https://github.com/firelink-data/padder/actions/workflows/ci.yml/badge.svg)](https://github.com/firelink-data/padder/actions/workflows/ci.yml)
[![CD](https://github.com/firelink-data/padder/actions/workflows/cd.yml/badge.svg)](https://github.com/firelink-data/padder/actions/workflows/cd.yml)
[![Tests](https://github.com/firelink-data/padder/actions/workflows/tests.yml/badge.svg)](https://github.com/firelink-data/padder/actions/workflows/tests.yml)

⚡️*Highly efficient data and string formatting library for Rust.*

</div>


## 🔎 Overview

Pad and format virtually any generic slice or vector efficiently with minimal memory overhead. This crate has guaranteed performance improvements over the standard
library `format!` macro. Clone this repository and run `cargo bench` to see benchmark comparisons between this implementation and the standard library.

The library defines a core trait called `Source` which enables efficient padding on the type. It is currently implemented on three main types of datastructures:
the string slice `&str`, the generic slice `&[T]`, and also the generic vector `Vec<T>`. Note that the type `T` has to adhere to the trait bound `T: From<Symbol>`,
where `Symbol` is the Enum representing the available characters/symbols to pad and format with. If you want to extend the padding capabilities of the `Source` trait
with your own type `T`, then you need to also implement the `From<Symbol>` trait for your type `T`. See the implementations for examples on how to do this, [link](https://github.com/firelink-data/padder/blob/main/src/lib.rs).


## 📦 Installation

The easiest way to include *padder* in your own crate is by using the [Cargo](https://crates.io/) package manager.
```
$ cargo add padder
```

Alternatively, you can build from source by cloning this repo and compiling using Cargo and then linking the library to your project.
```
$ git clone https://github.com/firelink-data/padder.git
$ cd padder
$ cargo build --release
```


## 🚀 Examples

Adding *padder* to your crate dependecy will bring the `Source` trait into scope and allow padding. 

You can for example pad string slices very easily in the following way:

```rust
let padded: String = "cool".pad(10, Alignment::Center, Symbol::Zero);
```

which would produce the padded String `000cool000`. You can also pad to an already allocated buffer, granting you full control of any heap allocations, like below:

```rust
let width: usize = 8;
let mut output: Vec<u8> = Vec::with_capacity(width);
let original = vec![13u8, 9, 128, 81];
original.pad_and_push_to_buffer(width, Alignment::Right, Symbol::Whitespace, output);
```

There also exists two wrapper methods simply called `pad` and `pad_and_push_to_buffer` which allows padding on any type as long as it implements the `Source` trait.
You can for example use these functions like below:
```rust
// pad
let original: &str = "hej";
let width: usize = 9;
let output: String = pad(original, width, Alignment::Left, Symbol::Hyphen);

// pad_and_push_to_buffer
let buffer = String::with_capacity(11);
pad_and_push_to_buffer("testcool", 11, Alignment::Right, Symbol::Whitespace);
```

which would produce the strings `hej------` and `   testcool`.

## 📋 License
All code is to be held under a general MIT license, please see [LICENSE](https://github.com/firelink-data/padder/blob/main/LICENSE) for specific information.
