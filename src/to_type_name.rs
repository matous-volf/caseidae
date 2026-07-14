use crate::common::ConstructRawHandled;
use convert_case::Case;
use syn::Ident;

use crate::common::ConvertCasing;

/// Types convertible to the letter casing corresponding to names of
/// [types](https://doc.rust-lang.org/reference/types.html).
pub trait ToTypeName {
    /// The resulting type, coming out of the conversion.
    type Output;
    /// Converts self to the letter casing of [type](https://doc.rust-lang.org/reference/types.html)
    /// names (`PascalCase`). Adheres to the [general conversion
    /// rules](crate#general-rules-for-conversions).
    ///
    /// This trait also provides three methods that are just aliases to this one:
    /// [`to_struct_name`](ToTypeName::to_struct_name), [`to_enum_name`](ToTypeName::to_enum_name),
    /// [`to_union_name`](ToTypeName::to_union_name). They should be used if the result of the
    /// conversion is known to be used for the specific item, so that the intent of the conversion
    /// is clear when reading the code. For instance, when a macro always generates a struct, it is
    /// more descriptive to use [`to_struct_name`](ToTypeName::to_struct_name). If you can't be sure
    /// whether the identifier will truly be of a struct (for example if it might as well later turn
    /// out to be for an enum), it's better to use this method instead.
    ///
    /// # Examples
    ///
    /// Usage with [`syn::Ident`]:
    ///
    /// ```
    /// # use caseidae::ToTypeName;
    /// # use syn::Ident;
    /// # use proc_macro2::Span;
    /// let variable_name = Ident::new("player_movement", Span::call_site());
    /// let type_name = variable_name.to_type_name();
    /// assert_eq!(type_name, "PlayerMovement");
    /// ```
    ///
    /// Usage with strings:
    ///
    /// ```
    /// # use caseidae::ToTypeName;
    /// assert_eq!("GRAVITATIONAL_CONSTANT".to_type_name(), "GravitationalConstant");
    /// assert_eq!(String::from("alex_and_der_murder").to_type_name(), "AlexAndDerMurder");
    /// ```
    ///
    /// # Panics
    ///
    /// Whether this method panics depends on the implementator, see their documentation for that.
    /// However, implementations from this crate never panic.
    fn to_type_name(&self) -> Self::Output;

    /// A more descriptive alias of [`to_type_name`](ToTypeName::to_type_name) specifically for
    /// cases when the result will be used for a struct. See the aliased method's documentation for
    /// guidance on when to use this.
    fn to_struct_name(&self) -> Self::Output {
        self.to_type_name()
    }

    /// A more descriptive alias of [`to_type_name`](ToTypeName::to_type_name) specifically for
    /// cases when the result will be used for an enum. See the aliased method's documentation for
    /// guidance on when to use this.
    fn to_enum_name(&self) -> Self::Output {
        self.to_type_name()
    }

    /// A more descriptive alias of [`to_type_name`](ToTypeName::to_type_name) specifically for
    /// cases when the result will be used for a union. See the aliased method's documentation for
    /// guidance on when to use this.
    fn to_union_name(&self) -> Self::Output {
        self.to_type_name()
    }
}

impl ToTypeName for str {
    type Output = <Self as ToOwned>::Owned;
    fn to_type_name(&self) -> Self::Output {
        self.convert_casing(Case::Pascal)
    }
}

#[cfg(feature = "syn")]
impl ToTypeName for Ident {
    type Output = <Self as ToOwned>::Owned;
    fn to_type_name(&self) -> Self::Output {
        Self::Output::new_raw_handled(self.to_string().to_type_name().as_str(), self.span())
    }
}
