use alloc::vec::Vec;
use proc_macro::TokenStream;
use syn::{Expr, Ident, ImplItem, Item, ItemImpl, ItemMod, Visibility};

/// Parses the contract module to extract the public struct, impl blocks, and `new` function.
///
/// This function performs three key tasks:
/// 1. Identifies the public struct in the module.
/// 2. Collects all implementation blocks and filters out the `new` function.
/// 3. Extracts the body of the `new` function for state initialization.
///
/// # Parameters
/// - `input_mod`: The mutable module to parse.
///
/// # Returns
/// - The name of the public struct.
/// - A vector of `impl` blocks with the `new` method removed.
/// - The body of the `new` function as an `Expr`.
///
/// # Errors
/// - If there is no public struct.
/// - If there is more than one public struct.
pub fn parse_contract(
    input_mod: &mut ItemMod,
) -> Result<(Ident, Vec<ItemImpl>, Option<Expr>), TokenStream> {
    let mut public_struct = None;
    let mut impl_blocks = Vec::new();
    let mut new_function_body = None;

    // Parse items in the module
    if let Some((_, items)) = &mut input_mod.content {
        for item in items.iter_mut() {
            match item {
                Item::Struct(s) => {
                    handle_public_struct(&mut public_struct, s)?;
                }
                Item::Impl(imp) => {
                    let (filtered_impl, new_body) = process_impl_block(imp)?;
                    impl_blocks.push(filtered_impl);
                    if new_function_body.is_none() {
                        new_function_body = new_body;
                    }
                }
                _ => {} // Ignore other items
            }
        }
    }

    // Unwrap the struct name because `handle_public_struct` ensures it exists
    let struct_name = public_struct.unwrap();

    Ok((struct_name, impl_blocks, new_function_body))
}

/// Handles the identification of the public struct.
///
/// Ensures that only one public struct is allowed in the module.
///
/// # Parameters
/// - `public_struct`: Option to store the struct name.
/// - `struct_item`: The struct item to process.
///
/// # Errors
/// - If more than one public struct is found.
fn handle_public_struct(
    public_struct: &mut Option<Ident>,
    struct_item: &syn::ItemStruct,
) -> Result<(), TokenStream> {
    if matches!(struct_item.vis, Visibility::Public(_)) {
        if public_struct.is_some() {
            return Err(syn::Error::new_spanned(
                struct_item,
                "Only one public struct is allowed in a contract module. Ensure your module defines exactly one public struct that serves as the contract's state.",
            )
            .to_compile_error()
            .into());
        }
        *public_struct = Some(struct_item.ident.clone());
    }
    Ok(())
}

/// Processes an `impl` block to filter out the `new` function and collect its body.
///
/// # Parameters
/// - `impl_block`: The implementation block to process.
///
/// # Returns
/// - The filtered implementation block without the `new` method.
/// - The body of the `new` function, if found.
fn process_impl_block(impl_block: &mut ItemImpl) -> Result<(ItemImpl, Option<Expr>), TokenStream> {
    let mut filtered_methods = Vec::new();
    let mut new_function_body = None;

    for item in &impl_block.items {
        if let ImplItem::Fn(func) = item {
            // Check if this method is the `new` function`
            if func.sig.ident == "new" {
                // Extract the first expression in the `new` function's body
                if let Some(stmt) = func.block.stmts.first() {
                    if let syn::Stmt::Expr(expr, _) = stmt {
                        new_function_body = Some(expr.clone());
                    }
                }
                continue; // Skip adding `new` to filtered methods
            }
        }
        // Add all other methods to the filtered list
        filtered_methods.push(item.clone());
    }

    impl_block.items = filtered_methods;
    Ok((impl_block.clone(), new_function_body))
}
