//! Contains the implementation of the `#[dusk_forge::contract]` macro.

use alloc::vec::Vec;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ImplItem, ItemImpl, Type, TypePath, Visibility};

/// Exposes functions in a smart contract's implementation block by
/// automatically generating external call wrappers.
///
/// This macro processes each public function in the contract's implementation
/// block, generating corresponding wrapper functions that facilitate calls from
/// the Dusk VM.
///
/// # Usage
///
/// Simply annotate your contract's `impl` block with `#[contract]`.
///
/// ```rust
/// #[contract]
/// impl MyContract {
///     pub fn my_method(&self) {
///         // Implementation...
///     }
/// }
/// ```
///
/// For each public method, a corresponding `no_mangle` function is generated.
/// Instance methods (`&self`, `&mut self`) assume `STATE` as the state holder
/// for your contract. Static methods on the implementation block are also
/// supported and are called directly on the type.
pub fn expand_contract(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the annotated implementation block
    let input_impl = parse_macro_input!(item as ItemImpl);
    // Extract the type for which the implementation is written. Used
    // specifically for generating correct function calls for static methods.
    let impl_type = if let Type::Path(TypePath { path, .. }) = &*input_impl.self_ty {
        path.segments.last().unwrap().ident.clone()
    } else {
        panic!("Expected a type path for the impl block");
    };

    // Vec to store the generated "wrap_call" functions for each public function
    // on the impl block
    let mut generated_functions = Vec::new();

    for item in &input_impl.items {
        // If the item within an impl block is a public function, continue
        if let ImplItem::Fn(method) = item {
            if matches!(method.vis, Visibility::Public(_)) {
                let method_name = &method.sig.ident;
                let is_instance_method = method
                    .sig
                    .inputs
                    .iter()
                    .any(|arg| matches!(arg, FnArg::Receiver(_)));

                // Process function arguments. For instance methods, skip the
                // first argument (`self`), for static methods, include all
                // arguments.
                let (arg_types, arg_pattern): (Vec<_>, Vec<_>) = method
                    .sig
                    .inputs
                    .iter()
                    .enumerate()
                    .filter_map(|(i, arg)| {
                        if i == 0 && is_instance_method {
                            None
                        } else {
                            match arg {
                                FnArg::Typed(pat_type) => {
                                    Some((pat_type.ty.clone(), format_ident!("arg{}", i)))
                                }
                                _ => panic!("Expected typed argument"),
                            }
                        }
                    })
                    .unzip();

                // Depending on whether it's an instance method or static
                // method, generate the call block on either "STATE" or directly
                // on the type.
                let call_block = if is_instance_method {
                    quote! { STATE.#method_name(#(#arg_pattern),*) }
                } else {
                    quote! { #impl_type::#method_name(#(#arg_pattern),*) }
                };

                // Generate the final "wrap_call" function
                let generated_function = quote! {
                    #[no_mangle]
                    pub unsafe fn #method_name(arg_len: u32) -> u32 {
                        dusk_core::abi::wrap_call(arg_len, |(#(#arg_pattern),*): (#(#arg_types),*)| #call_block)
                    }
                };

                generated_functions.push(generated_function);
            }
        }
    }

    // Combine the original implementation block with the generated "wrap_call"
    // functions
    let expanded = quote! {
        #input_impl

        #(#generated_functions)*
    };

    TokenStream::from(expanded)
}
