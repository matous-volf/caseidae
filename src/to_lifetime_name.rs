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
    ///
    /// # Warning
    ///
    /// The implementation on [`syn::Ident`] currently comes with a small caveat, see the
    /// [documentation there](trait.ToLifetimeName.html#warning-1).
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
    ///
    /// # Warning
    ///
    /// May return a [`Lifetime`](syn::Lifetime) which has an [`Ident`](syn::Ident) that would not
    /// actually be accepted as valid by [`Lifetime::new`](syn::Lifetime::new). This happens when
    /// the input [`Ident`](syn::Ident) is
    /// [raw](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html). Syn
    /// currently doesn't support creating [raw
    /// lifetimes](https://doc.rust-lang.org/edition-guide/rust-2021/raw-lifetimes.html) through
    /// [`Lifetime::new`](syn::Lifetime::new), so the [`Lifetime`](syn::Lifetime) is constructed
    /// with a stripped [`Ident`](syn::Ident) at first and
    /// then the `r#` prefix is artificially prepended, so that the original rawness is preserved.
    /// This warning will hopefully be removed in the future, when the feature gets implemented in
    /// syn. See the [related issue](https://codeberg.org/matous-volf/caseidae/issues/2).
    fn to_lifetime_name(&self) -> Self::Output {
        use crate::common::{ConstructRawHandled, IsRaw};

        let lifetime_name = self.to_string().to_lifetime_name();
        let is_raw = lifetime_name.is_raw_lifetime();
        let lifetime_name_unrawed = if is_raw {
            format!("{LIFETIME_TICK_PREFIX}{}", &lifetime_name[3..])
        } else {
            lifetime_name.clone()
        };
        let lifetime = syn::Lifetime::new(lifetime_name_unrawed.as_str(), self.span());

        if is_raw {
            // See the warning in the docs.
            syn::Lifetime {
                ident: Self::new_raw_handled(&lifetime_name[1..], self.span()),
                ..lifetime
            }
        } else {
            lifetime
        }
    }
}
