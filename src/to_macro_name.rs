use crate::common::{ConstructRawHandled, ConvertCasing};
use convert_case::Case;
use syn::Ident;

/// Types convertible to the letter casing corresponding to names of
/// [macros](https://doc.rust-lang.org/reference/macros.html).
pub trait ToMacroName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [macro](https://doc.rust-lang.org/reference/macros.html) names (`snake_case`).
    /// Adheres to the [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToMacroName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let trait_name = Ident::new("PartialEq", Span::call_site());
    /// let macro_name = trait_name.to_macro_name();
    /// assert_eq!(macro_name, "partial_eq");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToMacroName;
    /// assert_eq!("HTTPS_HRYZIKO_DEV".to_macro_name(), "https_hryziko_dev");
    /// assert_eq!(String::from("AlexAndDerMurder").to_macro_name(), "alex_and_der_murder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_macro_name(&self) -> Self::Output;
}

impl ToMacroName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_macro_name(&self) -> Self::Output {
        self.convert_casing(Case::Snake)
    }
}

#[cfg(feature = "syn")]
impl ToMacroName for Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_macro_name(&self) -> Self::Output {
        Self::Output::new_raw_handled(self.to_string().to_macro_name().as_str(), self.span())
    }
}
