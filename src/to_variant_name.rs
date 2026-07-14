use crate::common::ConstructRawHandled;
use convert_case::Case;
use syn::Ident;

use crate::common::ConvertCasing;

/// Types convertible to the letter casing corresponding to names of
/// [enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html) variants.
pub trait ToVariantName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html) variant names
    /// (`PascalCase`). Adheres to the [general conversion
    /// rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToVariantName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let function_name = Ident::new("paint_color", Span::call_site());
    /// let variant_name = function_name.to_variant_name();
    /// assert_eq!(variant_name, "PaintColor");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToVariantName;
    /// assert_eq!("ONION_LAYER_COUNT".to_variant_name(), "OnionLayerCount");
    /// assert_eq!(String::from("alex_and_der_murder").to_variant_name(), "AlexAndDerMurder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_variant_name(&self) -> Self::Output;
}

impl ToVariantName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_variant_name(&self) -> Self::Output {
        self.convert_casing(Case::Pascal)
    }
}

#[cfg(feature = "syn")]
impl ToVariantName for Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_variant_name(&self) -> Self::Output {
        Self::Output::new_raw_handled(self.to_string().to_variant_name().as_str(), self.span())
    }
}
