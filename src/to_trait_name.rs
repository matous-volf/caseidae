use crate::common::ConstructRawHandled;
use convert_case::Case;
use syn::Ident;

use crate::common::ConvertCasing;

/// Types convertible to the letter casing corresponding to names of
/// [traits](https://doc.rust-lang.org/rust-by-example/trait.html).
pub trait ToTraitName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [trait](https://doc.rust-lang.org/rust-by-example/trait.html) names (`PascalCase`). Adheres
    /// to the [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToTraitName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let variable_name = Ident::new("rainbow_height", Span::call_site());
    /// let trait_name = variable_name.to_trait_name();
    /// assert_eq!(trait_name, "RainbowHeight");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToTraitName;
    /// assert_eq!("ASSISTANT_REGIONAL_MANAGER".to_trait_name(), "AssistantRegionalManager");
    /// assert_eq!(String::from("alex_and_der_murder").to_trait_name(), "AlexAndDerMurder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_trait_name(&self) -> Self::Output;
}

impl ToTraitName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_trait_name(&self) -> Self::Output {
        self.convert_casing(Case::Pascal)
    }
}

#[cfg(feature = "syn")]
impl ToTraitName for Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_trait_name(&self) -> Self::Output {
        Self::Output::new_raw_handled(self.to_string().to_trait_name().as_str(), self.span())
    }
}
