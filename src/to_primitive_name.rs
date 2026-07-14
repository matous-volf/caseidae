use crate::common::ConstructRawHandled;
use convert_case::Case;
use syn::Ident;

use crate::common::ConvertCasing;

/// Types convertible to the letter casing corresponding to names of
/// [primitives](https://doc.rust-lang.org/rust-by-example/primitives.html).
pub trait ToPrimitiveName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [primitive](https://doc.rust-lang.org/rust-by-example/primitives.html) names (`flatcase`).
    /// Adheres to the [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToPrimitiveName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let enum_name = Ident::new("ThreeStateBool", Span::call_site());
    /// let primitive_name = enum_name.to_primitive_name();
    /// assert_eq!(primitive_name, "threestatebool");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToPrimitiveName;
    /// assert_eq!("UNSIGNED_INT_256".to_primitive_name(), "unsignedint256");
    /// assert_eq!(String::from("AlexAndDerMurder").to_primitive_name(), "alexanddermurder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_primitive_name(&self) -> Self::Output;
}

impl ToPrimitiveName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_primitive_name(&self) -> Self::Output {
        self.convert_casing(Case::Flat)
    }
}

#[cfg(feature = "syn")]
impl ToPrimitiveName for Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_primitive_name(&self) -> Self::Output {
        Self::Output::new_raw_handled(self.to_string().to_primitive_name().as_str(), self.span())
    }
}
