use alloc::vec::Vec;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, ImplItem, ItemImpl, Pat, Visibility};

/// Generates `no_mangle` functions for all public methods in the provided implementation blocks.
///
/// This function iterates over each `impl` block, processes its methods, and generates
/// `no_mangle` functions for public methods.
///
/// # Parameters
/// - `impl_blocks`: A slice of `ItemImpl` representing the implementation blocks to process.
/// - `mod_name`: The name of the module containing the contract.
///
/// # Returns
/// A vector of token streams representing all generated `no_mangle` functions.
///
/// Private methods such as `private_helper` will not have wrappers generated.
pub fn generate_public_functions(impl_blocks: &[ItemImpl], mod_name: &Ident) -> Vec<TokenStream> {
    impl_blocks
        .iter()
        .flat_map(|imp| generate_no_mangle_functions(imp, mod_name))
        .collect()
}

/// Generates `no_mangle` functions for a single `impl` block.
///
/// Public methods in the `impl` block are converted into `no_mangle` functions
/// that interact with the Dusk VM. Each method is wrapped with a function to enable
/// VM interaction.
///
/// # Parameters
/// - `imp`: The `impl` block to process.
/// - `mod_name`: The name of the module containing the contract.
///
/// # Returns
/// A vector of token streams representing the generated `no_mangle` functions.
fn generate_no_mangle_functions(imp: &ItemImpl, mod_name: &Ident) -> Vec<TokenStream> {
    imp.items
        .iter()
        .filter_map(|item| match item {
            ImplItem::Fn(method) if is_public_method(method) && method.sig.ident != "new" => Some(
                generate_wrapper_function(method, imp.trait_.is_some(), mod_name),
            ),
            _ => None,
        })
        .collect()
}

/// Checks whether a method is public.
///
/// # Parameters
/// - `method`: A reference to the method to check.
///
/// # Returns
/// `true` if the method is public, `false` otherwise.
fn is_public_method(method: &syn::ImplItemFn) -> bool {
    matches!(method.vis, Visibility::Public(_))
}

/// Generates the `no_mangle` wrapper for a given method.
///
/// This function creates a wrapper function that interacts with the Dusk VM. It handles both
/// instance and static methods and prepares the function's arguments for the VM.
///
/// # Parameters
/// - `method`: The method for which to generate the wrapper.
/// - `is_trait_impl`: Whether the method belongs to a trait implementation.
/// - `mod_name`: The name of the module containing the contract.
///
/// # Returns
/// A token stream representing the `no_mangle` wrapper function.
fn generate_wrapper_function(
    method: &syn::ImplItemFn,
    is_trait_impl: bool,
    mod_name: &Ident,
) -> TokenStream {
    let method_name = &method.sig.ident;

    let is_instance_method = method
        .sig
        .inputs
        .iter()
        .any(|arg| matches!(arg, FnArg::Receiver(_)));

    // Process arguments, skipping `self` for instance methods
    let (arg_patterns, arg_types): (Vec<_>, Vec<_>) = method
        .sig
        .inputs
        .iter()
        .enumerate()
        .filter_map(|(i, arg)| {
            if i == 0 && is_instance_method {
                // Skip `self`
                None
            } else if let FnArg::Typed(pat_type) = arg {
                // Extract the name from the pattern
                if let Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                    Some((pat_ident.ident.clone(), pat_type.ty.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unzip();

    // Generate the call block (state-based or static)
    let call_block = if is_instance_method {
        if is_trait_impl {
            quote! {
                <#mod_name::STATE as #method.sig.ident>::#method_name(#mod_name::STATE, #(#arg_patterns),*)
            }
        } else {
            quote! { #mod_name::STATE.#method_name(#(#arg_patterns),*) }
        }
    } else {
        quote! { #mod_name::#method_name(#(#arg_patterns),*) }
    };

    // Generate the wrapper function
    quote! {
        // A `no_mangle` wrapper for the `#method_name` method.
        #[no_mangle]
        pub unsafe fn #method_name(arg_len: u32) -> u32 {
            dusk_core::abi::wrap_call(arg_len, |(#(#arg_patterns),*): (#(#arg_types),*)| #call_block)
        }
    }
}
