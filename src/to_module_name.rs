use crate::common::ConvertCasing;
use convert_case::Case;

/// Types convertible to the letter casing corresponding to names of
/// [modules](https://doc.rust-lang.org/rust-by-example/mod.html).
pub trait ToModuleName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of
    /// [module](https://doc.rust-lang.org/rust-by-example/mod.html) names (`snake_case`).
    /// Adheres to the [general conversion rules](crate#general-rules-for-conversions).
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToModuleName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let enum_name = Ident::new("DinosaurSpecies", Span::call_site());
    /// let module_name = enum_name.to_module_name();
    /// assert_eq!(module_name, "dinosaur_species");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToModuleName;
    /// assert_eq!("T_REX".to_module_name(), "t_rex");
    /// assert_eq!(String::from("AlexAndDerMurder").to_module_name(), "alex_and_der_murder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_module_name(&self) -> Self::Output;
}

impl ToModuleName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_module_name(&self) -> Self::Output {
        self.convert_casing(Case::Snake)
    }
}

#[cfg(feature = "syn")]
impl ToModuleName for syn::Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_module_name(&self) -> Self::Output {
        use crate::common::ConstructRawHandled;

        Self::Output::new_raw_handled(self.to_string().to_module_name().as_str(), self.span())
    }
}
