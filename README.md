# Dusk Forge

[![Repository](https://img.shields.io/badge/github-dusk%20forge-blueviolet?logo=github)](https://github.com/HDauven/dusk-forge)
[![Documentation](https://img.shields.io/badge/docs-dusk%20forge-blue?logo=rust)](https://docs.rs/dusk-forge/)

Dusk Forge is a framework designed to simplify the development of smart
contracts for the [Dusk blockchain](https://github.com/dusk-network/rusk/). It
provides macros to reduce boilerplate, allow developers to focus on implementing
core business logic.

⚠️ **This crate requires the nightly Rust compiler.**

## Usage

The main feature of Dusk Forge is the `#[contract]` attribute macro, which
automates boilerplate generation. Here's how to get started:

### Installation

Add `dusk_forge` as a dependency to your project:

```sh
cargo add dusk_forge
```

### Basic Example: Counter Contract

```rust
#![no_std]

use dusk_forge::contract;

#[contract]
pub mod counter {
    pub struct Counter {
        value: i64,
    }

    impl Counter {
        pub fn new() -> Self {
            Self { value: 0 }
        }

        pub fn read_value(&self) -> i64 {
            self.value
        }

        pub fn increment(&mut self) {
            let value = self.value + 1;
            self.value = value;
        }
    }
}
```

### What Happes Under the Hood?

The `#[contract]` macro transforms the module into a contract-ready state by:

1. Generating a `static mut STATE` for the contract's state:

```rust
pub(crate) static mut STATE: Counter = Counter { value: 0 };
```

2. Wrapping public methods with `no_mangle` functions for Dusk VM compatibility:

```rust
#[no_mangle]
pub unsafe fn read_value(arg_len: u32) -> u32 {
    dusk_core::abi::wrap_call(arg_len, |(): ()| counter::STATE.read_value())
}

#[no_mangle]
pub unsafe fn increment(arg_len: u32) -> u32 {
    dusk_core::abi::wrap_call(arg_len, |(): ()| counter::STATE.increment())
}
```

### Advanced Example: Struct with Complex State

```rust
#![no_std]

use dusk_forge::contract;

#[contract]
pub mod complex_contract {
    pub struct AccountManager {
        accounts: core::collections::BTreeMap<u64, i64>,
    }

    impl AccountManager {
        pub fn new() -> Self {
            Self {
                accounts: core::collections::BTreeMap::new(),
            }
        }

        pub fn add_account(&mut self, id: u64, balance: i64) {
            self.accounts.insert(id, balance);
        }

        pub fn get_balance(&self, id: u64) -> Option<i64> {
            self.accounts.get(&id).cloned()
        }
    }
}
```

## Testing

Dusk Force includes tests for macro transformations and tests for individual
components. Run tests using:

```sh
cargo test
```

## Release History

To see the release history for this crate, please see the
[CHANGELOG](./CHANGELOG.md) file.

## License

This code is licensed under the Mozilla Public License Version 2.0 (MPL-2.0).
Please see the [LICENSE](./LICENSE) for further details.

## Contribute

If you want to contribute to this project, feel free to open an issue, fork the
repository and open a pull request.
