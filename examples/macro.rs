use caseidae::{ToFunctionName, ToLifetimeName, ToTypeName, ToVariableName};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

/// See the crate's readme for a description of this function.
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

    let function_generic_type_parameters = functions.iter().map(|function| function.to_type_name());

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

fn main() {
    let function_token_stream = trait_to_function(
        Ident::new("HeroHealth", Span::call_site()),
        &[(
            Ident::new("MAGIC_RESISTANCE", Span::call_site()),
            Ident::new("u8", Span::call_site()),
        )],
        &[Ident::new("Armor", Span::call_site())],
        &[
            Ident::new("deal_damage", Span::call_site()),
            // `type` is a keyword, so we need a raw identifier. Caseidae will handle it properly.
            Ident::new_raw("type", Span::call_site()),
            // Caseidae will preserve leading underscores.
            Ident::new("__hidden_secret", Span::call_site()),
        ],
    );

    println!("{}", function_token_stream);
}
