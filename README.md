# freelist-rs

A simple and efficient free list implementation in Rust, designed for managing the allocation and recycling of numeric IDs. Free numbers are recycled as late as possible to ensure minimal reuse over time, based on a Last In, First Out (LIFO) policy.

## Features

* LIFO-based recycling: Ensures that recently freed IDs are reused last
* Customizable ID type: Supports any type that implements Copy, PartialEq, Debug, and From<usize>

## Installation

Add the following to your Cargo.toml file:

```toml
[dependencies]
freelist-rs = "^0.0.3"
```

## License

This library is licensed under the MIT License. See [LICENSE](LICENSE) for details.
