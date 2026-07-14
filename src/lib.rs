#![doc(html_logo_url = "https://files.matousvolf.cz/public/caseidae/logo.png")]
#![doc(html_favicon_url = "https://files.matousvolf.cz/public/caseidae/logo.png")]

//! Convenient crate converting conventional casings of Rust identifiers, conveying correct context.
//!
//! While there are several crates providing casing manipulation functionality, Caseidae is
//! specifically designed for converting identifiers. It has built-in support for [`syn::Ident`],
//! handles lifetime annotations, and preserves prefixes like raw identifiers (`r#`) and
//! underscores. See the [rules](#general-rules-for-conversions) for details.
//!
//! Additionally, general casing conversions don't convey the same _meaning_. For instance, instead
//! of exposing a function named `to_snake_case`, this crate provides the same conversion under
//! functions called [`to_variable_name`](ToVariableName::to_variable_name),
//! [`to_field_name`](ToFieldName::to_field_name),
//! [`to_function_name`](ToFunctionName::to_function_name)
//! and so on. A reader of the code doesn't have to think about what the casing looks like, or why
//! it was chosen in the particular case (pun accidental) -- the intent is clear.
//!
//! # Example
//!
//! Imagine we are writing a wild macro that generates functions based on traits, such that
//!
//! - the function name is the trait name,
//! - the parameters are the trait's associated constants,
//! - the generic type parameters are the trait's functions,
//! - the lifetime parameters are the trait's generic parameters.
//!
//! For instance, turning
//!
//! ```
//! trait HeroHealth<Armor> {
//!     const MAGIC_RESISTANCE: u8 = 20;
//!
//!     fn deal_damage();
//!     // `type` is a keyword, so we need a raw identifier. Caseidae will handle it properly.
//!     fn r#type();
//!     // Caseidae will preserve leading underscores.
//!     fn __hidden_secret();
//! }
//! ```
//!
//! into
//!
//! ```
//! fn hero_health<'armor, DealDamage, r#Type, __HiddenSecret>(
//!     magic_resistance: u8
//! ) {
//!     // ...
//! }
//! ```
//!
//! Then our code might look like this:
//!
//! ```
//! use caseidae::{ToFunctionName, ToLifetimeName, ToTypeName, ToVariableName};
//! use proc_macro2::{Span, TokenStream};
//! use quote::quote;
//! use syn::Ident;
//!
//! fn trait_to_function(
//!     name: Ident,
//!     // The tuple is for name and type.
//!     associated_constants: &[(Ident, Ident)],
//!     generic_type_parameters: &[Ident],
//!     functions: &[Ident],
//! ) -> TokenStream {
//!     let function_name = name.to_function_name();
//!
//!     let function_lifetime_parameters = generic_type_parameters
//!         .iter()
//!         .map(|parameter| parameter.to_lifetime_name());
//!
//!     let function_generic_type_parameters = functions
//!         .iter()
//!         .map(|function| function.to_type_name());
//!
//!     let function_parameters = associated_constants
//!         .iter()
//!         .map(|(constant_name, constant_type)| {
//!             let parameter_name = constant_name.to_variable_name();
//!             quote! (#parameter_name: #constant_type)
//!         });
//!
//!     quote! {
//!         fn #function_name<
//!             #(#function_lifetime_parameters,)*
//!             #(#function_generic_type_parameters,)*
//!         >(#(#function_parameters,)*) {
//!             // ...
//!         }
//!     }
//! }
//! ```
//!
//! You can test this in
//! [`examples/macro.rs`](https://codeberg.org/matous-volf/caseidae/src/branch/main/examples/macro.rs).
//!
//! # General rules for conversions
//!
//! All performed conversions follow a set of common rules.
//!
//! 1. Some prefixes and suffixes are preserved:
//!
//!     - Leading
//!       [raw identifier](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html)
//!       prefixes (`r#`), only the first level (e.g. [`to_struct_name`](ToTypeName::to_struct_name)
//!       turns `r#r#r#oar` into `r#RROar`).
//!     - Leading or trailing underscores (`_`), any number of them. In particular, this means that
//!       [underscore
//!       expressions](https://doc.rust-lang.org/reference/expressions/underscore-expr.html) or
//!       [wildcard patterns](https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern)
//!       don't get destroyed.
//!
//!    In order for both of these to be preserved, they must appear in the correct order (`r#` and
//!    then some number of `_`). Note that if a single tick (`'`) is present at the start of the
//!    input, it is considered a lifetime prefix and is trimmed before detecting `r#` and `_`. Thus,
//!    for example, `'r#_fn` is turned into `r#_Fn` by
//!    [`to_trait_name`](ToTraitName::to_trait_name). However, whitespace or any other characters
//!    are not trimmed.
//!
//! 2. The input is split into words (later concatenated according to the output casing) by a set of
//!    boundaries:
//!
//!     - If the input matches screaming snake case, then the only boundary is underscore.
//!     - Otherwise, the boundaries are
//!         - any character that is not [ASCII alphanumeric](char::is_ascii_alphanumeric),
//!         - an uppercase letter,
//!         - a digit followed by a letter,
//!         - a letter followed by a digit.
//!
//!    In particular, this means that:
//!     - Outside of screaming case conversions, sequences of uppercase letters are considered
//!       sequences of one-letter words. For example, `TCPServer` is turned into `t_c_p_server` by
//!       [`to_field_name`](ToFieldName::to_field_name).
//!     - Numbers (sequences of digits) are considered separate words. For example,
//!       `vector180rotation` is turned into `Vector180Rotation` by
//!       [`to_enum_name`](ToTypeName::to_enum_name) and into `vector_180_rotation` by
//!       [`to_function_name`](ToFunctionName::to_function_name).
//!
//! 3. The output is not checked to be a valid Rust identifier. The implementations on
//!    [`syn::Ident`] are checked implicitly by the type, but still can collide with keywords. The
//!    implementations on `str` perform no checks whatsoever. For some use cases this is not a
//!    problem, for instance it's okay if a macro generates invalid code, you get a compilation
//!    error and fix the inputs. If this is not acceptable, you might utilize a crate like
//!    [check_keyword](https://crates.io/crates/check_keyword),
//!    [convert_string](https://crates.io/crates/convert_string), or similar.

mod common;

mod to_attribute_name;
mod to_constant_name;
mod to_crate_name;
mod to_field_name;
mod to_function_name;
mod to_lifetime_name;
mod to_macro_name;
mod to_module_name;
mod to_primitive_name;
mod to_static_name;
mod to_trait_name;
mod to_type_name;
mod to_variable_name;
mod to_variant_name;

pub use to_attribute_name::*;
pub use to_constant_name::*;
pub use to_crate_name::*;
pub use to_field_name::*;
pub use to_function_name::*;
pub use to_lifetime_name::*;
pub use to_macro_name::*;
pub use to_module_name::*;
pub use to_primitive_name::*;
pub use to_static_name::*;
pub use to_trait_name::*;
pub use to_type_name::*;
pub use to_variable_name::*;
pub use to_variant_name::*;

#[doc = include_str!("../readme.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
