use std::iter;

use convert_case::{Boundary, Case, Casing};
use convert_case_extras::is_case;

pub(crate) const RAW_IDENTIFIER_PREFIX: &str = "r#";
pub(crate) const LIFETIME_TICK_PREFIX: char = '\'';
pub(crate) const UNDERSCORE: char = '_';

pub(crate) const SNAKE_PASCAL_BOUNDARIES: [Boundary; 6] = [
    Boundary::DigitLower,
    Boundary::DigitUpper,
    Boundary::LowerDigit,
    Boundary::UpperDigit,
    Boundary::Custom {
        condition: |str| {
            str.first()
                .and_then(|str| str.chars().next())
                .is_some_and(|char| !char.is_ascii_alphanumeric())
        },
        start: 0,
        len: 1,
    },
    // Seperate by any uppercase, so that e.g. `ABC` is separated into three words.
    Boundary::Custom {
        condition: |str| {
            str.get(1)
                .and_then(|str| str.chars().next())
                .is_some_and(char::is_uppercase)
        },
        start: 1,
        len: 0,
    },
];

pub(crate) trait ConvertCasing {
    type Output;
    fn convert_casing(&self, casing: Case) -> Self::Output;
}

impl ConvertCasing for str {
    type Output = String;

    fn convert_casing(&self, casing: Case) -> Self::Output {
        // `self` can't be shadowed.
        let identifier = self;
        // Trim lifetime tick.
        let identifier = identifier
            .strip_prefix(LIFETIME_TICK_PREFIX)
            .unwrap_or(identifier);
        let (identifier, is_raw) = identifier
            .strip_prefix(RAW_IDENTIFIER_PREFIX)
            .map(|rest| (rest, true))
            .unwrap_or((identifier, false));
        let leading_underscore_count = identifier
            .chars()
            .take_while(|&char| char == UNDERSCORE)
            .count();
        let trailing_underscore_count = if leading_underscore_count < identifier.len() {
            identifier
                .chars()
                .rev()
                .take_while(|&char| char == UNDERSCORE)
                .count()
        } else {
            0
        };
        let identifier =
            &identifier[leading_underscore_count..identifier.len() - trailing_underscore_count];

        let boundaries = if is_case(identifier, Case::Constant) {
            Case::Constant.boundaries()
        } else {
            &SNAKE_PASCAL_BOUNDARIES
        };
        let converter = identifier.remove_empty().set_boundaries(boundaries);

        is_raw
            .then_some(RAW_IDENTIFIER_PREFIX.chars())
            .into_iter()
            .flatten()
            .chain(iter::repeat_n(UNDERSCORE, leading_underscore_count))
            .chain(converter.to_case(casing).chars())
            .chain(iter::repeat_n(UNDERSCORE, trailing_underscore_count))
            .collect()
    }
}

#[cfg(feature = "syn")]
use proc_macro2::Span;
#[cfg(feature = "syn")]
use syn::Ident;

#[cfg(feature = "syn")]
pub(crate) trait ConstructRawHandled {
    fn new_raw_handled(string: &str, span: Span) -> Self;
}

#[cfg(feature = "syn")]
impl ConstructRawHandled for Ident {
    fn new_raw_handled(string: &str, span: Span) -> Self {
        if string.is_raw_identifier() {
            Ident::new_raw(&string[2..], span)
        } else {
            Ident::new(string, span)
        }
    }
}

#[cfg(feature = "syn")]
pub(crate) trait IsRaw {
    fn is_raw_identifier(&self) -> bool;
    fn is_raw_lifetime(&self) -> bool;
}

#[cfg(feature = "syn")]
impl IsRaw for str {
    fn is_raw_identifier(&self) -> bool {
        self.starts_with(RAW_IDENTIFIER_PREFIX)
    }

    fn is_raw_lifetime(&self) -> bool {
        self.strip_prefix(LIFETIME_TICK_PREFIX)
            .is_some_and(|rest| rest.starts_with(RAW_IDENTIFIER_PREFIX))
    }
}

#[cfg(feature = "syn")]
impl IsRaw for Ident {
    fn is_raw_identifier(&self) -> bool {
        self.to_string().is_raw_identifier()
    }

    fn is_raw_lifetime(&self) -> bool {
        self.to_string().is_raw_lifetime()
    }
}

/// A check for the resolution of https://codeberg.org/matous-volf/caseidae/issues/2.
#[cfg(feature = "syn")]
#[test]
#[should_panic]
fn syn_raw_lifetime_check() {
    syn::Lifetime::new("'r#fn", Span::call_site());
}
