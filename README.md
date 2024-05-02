# Rust Option and Match Demonstrations

This repository contains a Rust script that demonstrates the use of the `Option<T>` enum and `match` expressions. It provides examples of how to handle nullable types safely without the risk of null pointer exceptions, showcasing the advantages of Rust's type system and control flow constructs.

## Overview

Rust's `Option<T>` type is used to represent a value that can either be something (`Some(T)`) or nothing (`None`). This project illustrates how to handle such types using `match` expressions, which are a powerful control flow feature in Rust. The code covers:

- Defining and using enums (`Coin` and `UsState`).
- Pattern matching with `match`.
- Safe handling of nullable types with `Option<T>`.
- Using `if let` for more concise pattern matching.

## Features

- **Enums and Pattern Matching**: Demonstrates defining and using enums, along with pattern matching to execute different logic based on the enum's current value.
- **Handling Nullable Types**: Uses `Option<T>` to handle cases where a value may or may not be present, illustrating how Rust helps avoid null pointer exceptions.
- **Control Flow Simplification**: Shows how `if let` can be used to simplify control flow when only one pattern is of interest.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your machine. If Rust is not installed, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
