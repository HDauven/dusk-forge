//! `dusk-forge` is a smart contract development framework designed to simplify the development of
//! smart contracts for the Dusk virtual machine. It provides macros to
//! automatically generate the boilerplate code required for interfacing smart
//! contracts with the Dusk VM.

#![feature(proc_macro_quote)]
#![no_std]
extern crate alloc;
extern crate proc_macro;

mod contract;

/// Procedural macro for the `#[dusk_forge::contract]` attribute.
#[proc_macro_attribute]
pub fn contract(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    contract::expand_contract(attr, item)
}
