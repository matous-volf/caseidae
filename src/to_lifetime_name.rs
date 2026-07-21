use crate::common::{ConvertCasing, LIFETIME_TICK_PREFIX};
use convert_case::Case;

/// Types convertible to the letter casing corresponding to names of
/// [lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html).
pub trait ToLifetimeName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [lifetime](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html) names
    /// (`'snake_case`). Adheres to the
    /// [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToLifetimeName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let enum_name = Ident::new("HeroAbility", Span::call_site());
    /// let lifetime = enum_name.to_lifetime_name();
    /// assert_eq!(lifetime.to_string(), "'hero_ability");
    /// ```
    ///
    /// Note that the output of the conversion (here the `lifetime` variable) is of type
    /// [`syn::Lifetime`].
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToLifetimeName;
    /// assert_eq!("PIZZA_DIAMETER".to_lifetime_name(), "'pizza_diameter");
    /// assert_eq!(String::from("AlexAndDerMurder").to_lifetime_name(), "'alex_and_der_murder");
    /// assert_eq!(String::from("'already_a_lifetime").to_lifetime_name(), "'already_a_lifetime");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_lifetime_name(&self) -> Self::Output;
}

impl ToLifetimeName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_lifetime_name(&self) -> Self::Output {
        format!("{LIFETIME_TICK_PREFIX}{}", self.convert_casing(Case::Snake))
    }
}

#[cfg(feature = "syn")]
impl ToLifetimeName for syn::Ident {
    type Output = syn::Lifetime;
    /// See the [trait method documentation](ToLifetimeName::to_lifetime_name).
    ///
    /// The resulting [`Lifetime`](syn::Lifetime) has the same span as self.
    fn to_lifetime_name(&self) -> Self::Output {
        syn::Lifetime::new(self.to_string().to_lifetime_name().as_str(), self.span())
    }
}
