# outerspace-rs

[![Crates.io version](https://img.shields.io/crates/v/outerspace.svg?style=flat-square)](https://crates.io/crates/outerspace)
[![Crates.io downloads](https://img.shields.io/crates/d/outerspace.svg?style=flat-square)](https://crates.io/crates/outerspace)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/outerspace)

This is the Rust version of the [outerspace JavaScript package](https://www.npmjs.com/package/outerspace).

Methods for prefixing and suffixing the non-whitespace characters in a string.

# Examples

```rust
let prefixed = outerspace::prefix_non_whitespace("\n\nHello hello\n\n", "> ");
assert_eq!(prefixed, "\n\n> Hello hello\n\n");
```

```rust
let suffixed = outerspace::suffix_non_whitespace("\n\nHello hello\n\n", "!");
assert_eq!(suffixed, "\n\nHello hello!\n\n");
```

```rust
let wrapped = outerspace::wrap_non_whitespace("\n\nHello hello\n\n", "**", "**");
assert_eq!(wrapped, "\n\n**Hello hello**\n\n");
```
