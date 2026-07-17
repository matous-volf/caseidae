use crate::common::ConvertCasing;
use convert_case::Case;

/// Types convertible to the letter casing corresponding to names of
/// [constant items](https://doc.rust-lang.org/reference/items/constant-items.html).
pub trait ToConstantName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of [constant
    /// item](https://doc.rust-lang.org/reference/items/constant-items.html) names
    /// (`SCREAMING_SNAKE_CASE`). Adheres to the [general conversion
    /// rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToConstantName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let struct_name = Ident::new("SecondsToSelfDestruct", Span::call_site());
    /// let constant_name = struct_name.to_constant_name();
    /// assert_eq!(constant_name, "SECONDS_TO_SELF_DESTRUCT");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToConstantName;
    /// assert_eq!("emperor_zurg_max_health".to_constant_name(), "EMPEROR_ZURG_MAX_HEALTH");
    /// assert_eq!(String::from("AlexAndDerMurder").to_constant_name(), "ALEX_AND_DER_MURDER");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_constant_name(&self) -> Self::Output;
}

impl ToConstantName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_constant_name(&self) -> Self::Output {
        self.convert_casing(Case::Constant)
    }
}

#[cfg(feature = "syn")]
impl ToConstantName for syn::Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_constant_name(&self) -> Self::Output {
        use crate::common::ConstructRawHandled;

        Self::Output::new_raw_handled(self.to_string().to_constant_name().as_str(), self.span())
    }
}
