mod functions;
mod parser;
mod state;
mod transformation;

use functions::generate_public_functions;
use parser::parse_contract;
use state::generate_state_declaration;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

/// Expands the `#[dusk_forge::contract]` macro.
///
/// This macro simplifies smart contract development by:
/// 1. Parsing the annotated module to identify public structs, impl blocks,
///    and trait implementations.
/// 2. Generating `no_mangle` functions for all public methods in `impl` blocks.
/// 3. Automatically defining a `static mut STATE` for the module's public struct.
/// 4. Validating that the module contains exactly one public struct.
///
/// # Errors
/// If the module contains:
/// - No public struct.
/// - Multiple public structs.
/// The macro will return a compile-time error.
pub fn expand_contract(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_mod = parse_macro_input!(item as ItemMod);
    let mod_name = input_mod.ident.clone();

    // Parse contract components
    let (struct_name, impl_blocks, new_function_body) = match parse_contract(&mut input_mod) {
        Ok(data) => data,
        Err(err) => return err,
    };

    // Generate the state for the contract
    if let Err(err) = generate_state_declaration(&struct_name, new_function_body, &mut input_mod) {
        return err;
    }

    // Generate `no_mangle` functions for public methods
    let generated_functions = generate_public_functions(&impl_blocks, &mod_name);

    // Combine all pieces into the final output
    let expanded = quote! {
        #input_mod
        #(#generated_functions)*
    };

    expanded.into()
}
