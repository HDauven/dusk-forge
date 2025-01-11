# Dusk Forge

[![Repository](https://img.shields.io/badge/github-dusk%20forge-blueviolet?logo=github)](https://github.com/HDauven/dusk-forge)
[![Documentation](https://img.shields.io/badge/docs-dusk%20forge-blue?logo=rust)](https://docs.rs/dusk-forge/)

`dusk-forge` is a smart contract development framework designed to simplify the
development of smart contracts for the
[Dusk VM](https://github.com/dusk-network/rusk/tree/master/vm). It provides
macros to automatically generate the boilerplate code required for interfacing
smart contracts with the Dusk VM.

## Usage

The main feature of `dusk-forge` is the `#[contract]` attribute macro. This
macro automatically generates wrapper functions required for interfacing with
the Dusk VM, reducing boilerplate code in your project.

Add `dusk_forge` as a dependency to your contract project:

```sh
cargo add dusk_forge
```

To use the macro, import it into your Rust smart contract and annotate your
contract's implementation with #[contract]:

```rust
#![no_std]

use dusk_forge::contract;

/// Struct that describes the state of the Counter contract
pub struct Counter {
    value: i64,
}

/// State of the Counter contract
static mut STATE: Counter = Counter { value: 0xfc };

#[contract]
impl Counter {
    /// Read the value of the counter
    pub fn read_value(&self) -> i64 {
        self.value
    }

    /// Increment the value of the counter by 1
    pub fn increment(&mut self) {
        let value = self.value + 1;
        self.value = value;
    }
}
```

With #[contract], the macro automatically generates the necessary wrapper
functions for each public method in the impl block you want to expose.

## Release History

To see the release history for this crate, please see the
[CHANGELOG](./CHANGELOG.md) file.

## License

This code is licensed under the Mozilla Public License Version 2.0 (MPL-2.0).
Please see the [LICENSE](./LICENSE) for further details.

## Contribute

If you want to contribute to this project, feel free to open an issue, fork the
repository and open a pull request.
