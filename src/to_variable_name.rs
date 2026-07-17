use crate::common::ConvertCasing;
use convert_case::Case;

/// Types convertible to the letter casing corresponding to names of
/// [variables](https://doc.rust-lang.org/reference/variables.html).
pub trait ToVariableName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [variable](https://doc.rust-lang.org/reference/variables.html) names (`snake_case`).
    /// Adheres to the [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToVariableName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let struct_name = Ident::new("BinarySearchTree", Span::call_site());
    /// let variable_name = struct_name.to_variable_name();
    /// assert_eq!(variable_name, "binary_search_tree");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToVariableName;
    /// assert_eq!("BUGS_PER_LINE".to_variable_name(), "bugs_per_line");
    /// assert_eq!(String::from("AlexAndDerMurder").to_variable_name(), "alex_and_der_murder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_variable_name(&self) -> Self::Output;
}

impl ToVariableName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_variable_name(&self) -> Self::Output {
        self.convert_casing(Case::Snake)
    }
}

#[cfg(feature = "syn")]
impl ToVariableName for syn::Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_variable_name(&self) -> Self::Output {
        use crate::common::ConstructRawHandled;

        Self::Output::new_raw_handled(self.to_string().to_variable_name().as_str(), self.span())
    }
}
