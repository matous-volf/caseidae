use crate::common::ConvertCasing;
use convert_case::Case;

/// Types convertible to the letter casing corresponding to names of
/// [attributes](https://doc.rust-lang.org/rust-by-example/attribute.html).
pub trait ToAttributeName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [attribute](https://doc.rust-lang.org/rust-by-example/attribute.html) names (`snake_case`).
    /// Adheres to the [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToAttributeName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let struct_name = Ident::new("HeroHealth", Span::call_site());
    /// let attribute_name = struct_name.to_attribute_name();
    /// assert_eq!(attribute_name, "hero_health");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToAttributeName;
    /// assert_eq!("SPEED_OF_LIGHT".to_attribute_name(), "speed_of_light");
    /// assert_eq!(String::from("AlexAndDerMurder").to_attribute_name(), "alex_and_der_murder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_attribute_name(&self) -> Self::Output;
}

impl ToAttributeName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_attribute_name(&self) -> Self::Output {
        self.convert_casing(Case::Snake)
    }
}

#[cfg(feature = "syn")]
impl ToAttributeName for syn::Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_attribute_name(&self) -> Self::Output {
        use crate::common::ConstructRawHandled;

        Self::Output::new_raw_handled(self.to_string().to_attribute_name().as_str(), self.span())
    }
}
