use alloc::vec;
use syn::{punctuated::Punctuated, visit_mut, visit_mut::VisitMut, Ident, Path, PathSegment};

/// A transformation that replaces occurrences of `Self` with a specified struct name.
///
/// This is useful in scenarios where `Self` is not allowed (e.g., in static initialization of the state),
/// and we need to explicitly refer to the associated struct.
///
/// # Example
///
/// ## Input
/// ```rust
/// Self {
///     field1: 0,
///     field2: String::new(),
/// }
/// ```
///
/// ## Transformation
/// Struct name: `MyStruct`
///
/// ## Output
/// ```rust
/// MyStruct {
///     field1: 0,
///     field2: String::new(),
/// }
/// ```
pub struct ReplaceSelfWithStructName<'a> {
    pub struct_name: &'a Ident,
}

impl<'a> VisitMut for ReplaceSelfWithStructName<'a> {
    /// Visits mutable paths in the syntax tree and replaces `Self` with the struct name.
    ///
    /// This method is called recursively on all paths within the syntax tree.
    ///
    /// # Parameters
    /// - `path`: A mutable reference to a `Path` in the syntax tree.
    ///
    /// # Example Usage
    ///
    /// ```rust
    /// use syn::{parse_quote, Ident, Path};
    /// use crate::ReplaceSelfWithStructName;
    ///
    /// // Create a path with `Self`
    /// let mut path: Path = parse_quote! { Self };
    ///
    /// // Define the struct name to replace `Self`
    /// let struct_name: Ident = syn::parse_str("MyStruct").unwrap();
    ///
    /// // Apply the transformation
    /// ReplaceSelfWithStructName { struct_name: &struct_name }.visit_path_mut(&mut path);
    ///
    /// // Assert the result
    /// assert_eq!(path.to_token_stream().to_string(), "MyStruct");
    /// ```
    fn visit_path_mut(&mut self, path: &mut Path) {
        if path.is_ident("Self") {
            // Replace `Self` with the struct name
            path.segments =
                Punctuated::from_iter(vec![PathSegment::from(self.struct_name.clone())]);
        }
        // Continue visiting nested paths
        visit_mut::visit_path_mut(self, path);
    }
}
