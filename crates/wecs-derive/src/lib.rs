use proc_macro::{token_stream, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component, attributes(r))]
pub fn component(input: TokenStream) -> TokenStream {
    let token_stream = parse_macro_input!(input as DeriveInput);
    let ident = token_stream.ident.clone();
    let generics = token_stream.generics.clone();

    let impl_tokens = quote! {
        impl #generics ::wecs::component::Component for #ident #generics {}
    };

    TokenStream::from(impl_tokens)
}

#[proc_macro_derive(Resource)]
pub fn resource(input: TokenStream) -> TokenStream {
    let token_stream = parse_macro_input!(input as DeriveInput);
    let ident = token_stream.ident.clone();
    let generics = token_stream.generics.clone();

    quote! {
        impl #generics ::wecs::resource::Resource for #ident #generics { }
    }
    .into()
}
