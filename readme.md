# Caseidae

Convenient crate converting conventional casings of Rust identifiers, conveying
correct context.

While there are several crates providing casing manipulation functionality,
Caseidae is specifically designed for converting identifiers. It has built-in
support for [`syn::Ident`](https://docs.rs/syn/latest/syn/struct.Ident.html),
handles lifetime annotations, and preserves prefixes like raw identifiers (`r#`)
and underscores.

Additionally, general casing conversions don't convey the same _meaning_. For
instance, instead of exposing a function named `to_snake_case`, this crate
provides the same conversion under functions called
[`to_variable_name`](https://docs.rs/caseidae/latest/caseidae/trait.ToVariableName.html),
[`to_field_name`](https://docs.rs/caseidae/latest/caseidae/trait.ToFieldName.html),
[`to_function_name`](https://docs.rs/caseidae/latest/caseidae/trait.ToFunctionName.html)
and so on. A reader of the code doesn't have to think about what the casing
looks like, or why it was chosen in the particular case (pun accidental) – the
intent is clear.

## Example

Imagine we are writing a wild macro that generates functions based on traits,
such that

- the function name is the trait name,
- the parameters are the trait's associated constants,
- the generic type parameters are the trait's functions,
- the lifetime parameters are the trait's generic parameters.

For instance, turning

```rust
trait HeroHealth<Armor> {
    const MAGIC_RESISTANCE: u8 = 20;

    fn deal_damage();
    // `type` is a keyword, so we need a raw identifier.
    // Caseidae will handle it properly.
    fn r#type();
    // Caseidae will preserve leading underscores.
    fn __hidden_secret();
}
```

into

```rust
fn hero_health<'armor, DealDamage, r#Type, __HiddenSecret>(
    magic_resistance: u8
) {
    // ...
}
```

Then our code might look like this:

```rust
use caseidae::{ToFunctionName, ToLifetimeName, ToTypeName, ToVariableName};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

fn trait_to_function(
    name: Ident,
    // The tuple is for name and type.
    associated_constants: &[(Ident, Ident)],
    generic_type_parameters: &[Ident],
    functions: &[Ident],
) -> TokenStream {
    let function_name = name.to_function_name();

    let function_lifetime_parameters = generic_type_parameters
        .iter()
        .map(|parameter| parameter.to_lifetime_name());

    let function_generic_type_parameters = functions
        .iter()
        .map(|function| function.to_type_name());

    let function_parameters = associated_constants
        .iter()
        .map(|(constant_name, constant_type)| {
            let parameter_name = constant_name.to_variable_name();
            quote! (#parameter_name: #constant_type)
        });

    quote! {
        fn #function_name<
            #(#function_lifetime_parameters,)*
            #(#function_generic_type_parameters,)*
        >(#(#function_parameters,)*) {
            // ...
        }
    }
}
```

You can test this in
[`examples/macro.rs`](https://codeberg.org/matous-volf/caseidae/src/branch/main/examples/macro.rs).

## License

Licensed under the Mozilla Public License (MPL), version 2.0.
