use crate::common::ConstructRawHandled;
use convert_case::Case;
use syn::Ident;

use crate::common::ConvertCasing;

/// Types convertible to the letter casing corresponding to names of
/// [functions](https://doc.rust-lang.org/rust-by-example/fn.html).
pub trait ToFunctionName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [function](https://doc.rust-lang.org/rust-by-example/fn.html) names (`snake_case`).
    /// Adheres to the [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToFunctionName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let type_name = Ident::new("FallWithStyle", Span::call_site());
    /// let function_name = type_name.to_function_name();
    /// assert_eq!(function_name, "fall_with_style");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToFunctionName;
    /// assert_eq!("GOLDEN_RATIO".to_function_name(), "golden_ratio");
    /// assert_eq!(String::from("AlexAndDerMurder").to_function_name(), "alex_and_der_murder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_function_name(&self) -> Self::Output;
}

impl ToFunctionName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_function_name(&self) -> Self::Output {
        self.convert_casing(Case::Snake)
    }
}

#[cfg(feature = "syn")]
impl ToFunctionName for Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_function_name(&self) -> Self::Output {
        Self::Output::new_raw_handled(self.to_string().to_function_name().as_str(), self.span())
    }
}
