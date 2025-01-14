use crate::contract::transformation::ReplaceSelfWithStructName;
use proc_macro::TokenStream;
use syn::{visit_mut::VisitMut, Expr, Ident, ItemMod};

/// This function creates a `static mut STATE` variable, initialized using the `new` function
/// of the struct, and appends it to the module. It ensures that `Self` in the `new` function
/// body is replaced with the actual struct name, as `Self` is not valid in this context.
///
/// The `static mut STATE` variable serves as the state of a contract.
///
/// # Parameters
/// - `struct_name`: The name of the public struct.
/// - `new_function_body`: The body of the `new` function, which initializes the struct.
/// - `input_mod`: The mutable reference to the module where the `STATE` declaration is appended.
///
/// # Returns
/// - `Ok(())` if the `STATE` declaration is successfully added to the module.
/// - `Err(TokenStream)` if the `new` function is missing or invalid.
///
/// # Errors
/// This function returns an error if:
/// - The `new` function is not found.
/// - The `new` function's body is missing or invalid.
///
/// # Example
///
/// ## Input
/// ```rust
/// use dusk_forge::contract;
///
/// #[contract]
/// pub mod my_contract {
///     pub struct MyStruct {
///         field1: i64,
///         field2: String,
///     }
///
///     impl MyStruct {
///         pub fn new() -> Self {
///             Self {
///                 field1: 0,
///                 field2: String::new(),
///             }
///         }
///     }
/// }
/// ```
///
/// ## Generated Output
/// ```rust
/// pub mod my_contract {
///     pub struct MyStruct {
///         field1: i64,
///         field2: String,
///     }
///
///     impl MyStruct {
///         pub fn new() -> Self {
///             Self {
///                 field1: 0,
///                 field2: String::new(),
///             }
///         }
///     }
///
///     pub(crate) static mut STATE: MyStruct = MyStruct {
///         field1: 0,
///         field2: String::new(),
///     };
/// }
/// ```
pub fn generate_state_declaration(
    struct_name: &Ident,
    new_function_body: Option<Expr>,
    input_mod: &mut ItemMod,
) -> Result<(), TokenStream> {
    // Ensure the `new` function was found
    let mut transformed_body = match new_function_body {
        Some(body) => body,
        None => {
            return Err(syn::Error::new_spanned(
                input_mod,
                "The struct must implement a `new` function for initializing the contract state.",
            )
            .to_compile_error()
            .into());
        }
    };

    // Replace `Self` with the struct name in the `new` function body
    ReplaceSelfWithStructName { struct_name }.visit_expr_mut(&mut transformed_body);

    // Append the static state declaration to the module
    if let Some((_, items)) = &mut input_mod.content {
        // Holds the contract's state. This is automatically generated
        items.push(syn::parse_quote! {
            pub(crate) static mut STATE: #struct_name = #transformed_body;
        });
    }

    Ok(())
}
