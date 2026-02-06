# package_infos

A Rust library to retrieve and display package information and its dependencies.

## Overview

`package_infos` provides a struct `PackageInfos` and a macro `pkg_infos!` to easily gather metadata about a package (name, version, authors, description) from Cargo environment variables. It also allows defining dependencies which are also `PackageInfos` structures.

## Usage

Add `package_infos` to your `Cargo.toml`.

### Example

In your `lib.rs` or `main.rs`:

```rust
use package_infos::pkg_infos;

// Define package infos for dependencies if they also use package_infos
// mod dependency_crate;

// Generate package infos for the current crate
pkg_infos!();

// If you have dependencies that also expose get_package_infos():
// pkg_infos!(dependency_crate);

fn main() {
    let info = get_package_infos();
    println!("{}", info);
}
```

The `pkg_infos!` macro generates a `get_package_infos()` function that returns a `PackageInfos` struct. This struct implements `Display` to print the package details and its dependencies in a formatted way.

## Fields

The `PackageInfos` struct contains:
- `name`: Package name
- `version`: Package version
- `authors`: Package authors
- `description`: Package description
- `dependencies`: List of dependency `PackageInfos`

## Installation

```bash
cargo build
```

## Testing

```bash
cargo test
```
