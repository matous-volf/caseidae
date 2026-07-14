use crate::common::{ConstructRawHandled, ConvertCasing};
use convert_case::Case;
use syn::Ident;

/// Types convertible to the letter casing corresponding to names of
/// [crates](https://doc.rust-lang.org/rust-by-example/crates.html).
pub trait ToCrateName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [crate](https://doc.rust-lang.org/rust-by-example/crates.html) names (`snake_case`).
    /// Adheres to the [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToCrateName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let trait_name = Ident::new("DeriveHarvestype", Span::call_site());
    /// let crate_name = trait_name.to_crate_name();
    /// assert_eq!(crate_name, "derive_harvestype");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToCrateName;
    /// assert_eq!("THE_NEVERHOOD".to_crate_name(), "the_neverhood");
    /// assert_eq!(String::from("AlexAndDerMurder").to_crate_name(), "alex_and_der_murder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_crate_name(&self) -> Self::Output;
}

impl ToCrateName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_crate_name(&self) -> Self::Output {
        self.convert_casing(Case::Snake)
    }
}

#[cfg(feature = "syn")]
impl ToCrateName for Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_crate_name(&self) -> Self::Output {
        Self::Output::new_raw_handled(self.to_string().to_crate_name().as_str(), self.span())
    }
}
