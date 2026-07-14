use crate::common::{ConstructRawHandled, ConvertCasing};
use convert_case::Case;
use syn::Ident;

/// Types convertible to the letter casing corresponding to names of
/// [static items](https://doc.rust-lang.org/reference/items/static-items.html).
pub trait ToStaticName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of [static
    /// item](https://doc.rust-lang.org/reference/items/static-items.html) names
    /// (`SCREAMING_SNAKE_CASE`). Adheres to the [general conversion
    /// rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToStaticName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let struct_name = Ident::new("MemoryLeakedMb", Span::call_site());
    /// let static_name = struct_name.to_static_name();
    /// assert_eq!(static_name, "MEMORY_LEAKED_MB");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToStaticName;
    /// assert_eq!("earth_moon_distance".to_static_name(), "EARTH_MOON_DISTANCE");
    /// assert_eq!(String::from("AlexAndDerMurder").to_static_name(), "ALEX_AND_DER_MURDER");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_static_name(&self) -> Self::Output;
}

impl ToStaticName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_static_name(&self) -> Self::Output {
        self.convert_casing(Case::Constant)
    }
}

#[cfg(feature = "syn")]
impl ToStaticName for Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_static_name(&self) -> Self::Output {
        Self::Output::new_raw_handled(self.to_string().to_static_name().as_str(), self.span())
    }
}
