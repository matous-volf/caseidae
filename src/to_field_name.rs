use crate::common::ConvertCasing;
use convert_case::Case;

/// Types convertible to the letter casing corresponding to names of fields.
pub trait ToFieldName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of field names (`snake_case`).
    /// Adheres to the [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToFieldName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let struct_name = Ident::new("PotatoesTomatoesGravyPeas", Span::call_site());
    /// let field_name = struct_name.to_field_name();
    /// assert_eq!(field_name, "potatoes_tomatoes_gravy_peas");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToFieldName;
    /// assert_eq!("EULERS_NUMBER".to_field_name(), "eulers_number");
    /// assert_eq!(String::from("AlexAndDerMurder").to_field_name(), "alex_and_der_murder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_field_name(&self) -> Self::Output;
}

impl ToFieldName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_field_name(&self) -> Self::Output {
        self.convert_casing(Case::Snake)
    }
}

#[cfg(feature = "syn")]
impl ToFieldName for syn::Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_field_name(&self) -> Self::Output {
        use crate::common::ConstructRawHandled;

        Self::Output::new_raw_handled(self.to_string().to_field_name().as_str(), self.span())
    }
}
