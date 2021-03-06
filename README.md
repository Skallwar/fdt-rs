# fdt-rs

[![crates.io](https://img.shields.io/crates/v/fdt-rs)](https://crates.io/crates/fdt-rs)
[![downloads](https://img.shields.io/crates/d/fdt-rs.svg)](https://crates.io/crates/fdt-rs)
[![docs.rs](https://docs.rs/fdt-rs/badge.svg)](https://docs.rs/fdt-rs/)
[![master](https://github.com/rs-embedded/fdt-rs/workflows/Build%20and%20test/badge.svg?branch=master)](https://github.com/rs-embedded/fdt-rs/actions)
[![coveralls.io](https://coveralls.io/repos/github/rs-embedded/fdt-rs/badge.svg)](https://coveralls.io/github/rs-embedded/fdt-rs)

A Flattened Device Tree parser for embedded no-std environments

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.fdt-rs]
version = "0.4"
```

and this to your crate root:

```rust
extern crate fdt_rs;
```

## Features

This crate can be used without the standard library (`#![no_std]`) by disabling
the default `std` feature. Use this in `Cargo.toml`:

```toml
[dependencies.fdt-rs]
version = "0.4"
default-features = false
```

## Example

The following example stashes a flattened device tree in memory, parses that
device tree into a `fdt_rs::DevTree` object, searches the device tree for
"ns16550a" compatible nodes, and (if found) prints each nodes' name.

```rust
extern crate fdt_rs;
use fdt_rs::prelude::*;
use fdt_rs::base::*;

// Place a device tree image into the rust binary and
// align it to a 32-byte boundary by using a wrapper struct.
#[repr(align(4))] struct _Wrapper<T>(T);
pub const FDT: &[u8] = &_Wrapper(*include_bytes!("../tests/riscv64-virt.dtb")).0;

fn main() {
    // Initialize the devtree using an &[u8] array.
    let devtree = unsafe {

        // Get the actual size of the device tree after reading its header.
        let size = DevTree::read_totalsize(FDT).unwrap();
        let buf = &FDT[..size];

        // Create the device tree handle
        DevTree::new(buf).unwrap()
    };

    // Iterate through all "ns16550a" compatible nodes within the device tree.
    // If found, print the name of each node (including unit address).
    let mut node_iter = devtree.compatible_nodes("ns16550a");
    while let Some(node) = node_iter.next().unwrap() {
        println!("{}", node.name().unwrap());
    }
}

```

Also check out [fdtdump](https://github.com/rs-embedded/fdtdump) for an example implementation of the fdtdump Device Tree utility using this library.
