//! Derive explicit sub-state extraction for `app-context`.
//!
//! Prefer `app_context::FromContext` with the core crate's `derive` feature.
//! This macro supports named, tuple and unit structs, including generics. Fields
//! use `FromContext` by default. `#[context(clone)]` clones a registered concrete
//! value; `#[context(default)]` uses `Default` without consulting the context.
//! Enums, unions, container attributes and unknown field options are rejected.
//! Renamed `app-context` dependencies are resolved automatically.
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, parse_quote};

/// Derive `app_context::FromContext` for an explicit sub-state struct.
///
/// Expansion performs field extraction in declaration order using `?`. A failed
/// later field does not undo earlier clones or user-defined extraction effects.
/// This generates no factories, global state, unsafe code or hidden caching.
#[proc_macro_derive(FromContext, attributes(context))]
pub fn derive_from_context(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    for attr in &input.attrs {
        if attr.path().is_ident("context") {
            return Err(syn::Error::new_spanned(
                attr,
                "context options are only supported on fields",
            ));
        }
    }
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "FromContext can only be derived for structs",
            ));
        }
    };
    let core = match crate_name("app-context") {
        Ok(FoundCrate::Itself) => quote!(::app_context),
        Ok(FoundCrate::Name(name)) => {
            let name = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote!(::#name)
        }
        Err(error) => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                format!("FromContext requires an app-context dependency: {error}"),
            ));
        }
    };
    let mut generics = input.generics.clone();
    let mut values = Vec::new();
    for field in fields {
        let ty = &field.ty;
        let mut option = None;
        for attr in &field.attrs {
            if !attr.path().is_ident("context") {
                continue;
            }
            if !matches!(&attr.meta, syn::Meta::List(_)) {
                return Err(syn::Error::new_spanned(
                    attr,
                    "expected #[context(clone)] or #[context(default)]",
                ));
            }
            let mut count = 0;
            attr.parse_nested_meta(|meta| {
                count += 1;
                if option.is_some() {
                    return Err(meta.error("only one context option is allowed per field"));
                }
                if meta.path.is_ident("clone") {
                    option = Some(true);
                } else if meta.path.is_ident("default") {
                    option = Some(false);
                } else {
                    return Err(meta.error("supported context options are clone and default"));
                }
                Ok(())
            })?;
            if count == 0 {
                return Err(syn::Error::new_spanned(
                    attr,
                    "context requires clone or default",
                ));
            }
        }
        let expression = match option {
            Some(true) => {
                generics.make_where_clause().predicates.push(parse_quote!(#ty: ::core::clone::Clone + ::core::marker::Send + ::core::marker::Sync + 'static));
                quote!(<#ty as ::core::clone::Clone>::clone(context.get_ref::<#ty>()?))
            }
            Some(false) => {
                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(#ty: ::core::default::Default));
                quote!(<#ty as ::core::default::Default>::default())
            }
            None => {
                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(#ty: #core::FromContext));
                quote!(<#ty as #core::FromContext>::from_context(context)?)
            }
        };
        values.push(expression);
    }
    let construction = match fields {
        Fields::Named(named) => {
            let names = named.named.iter().map(|f| &f.ident);
            quote!(Self { #(#names: #values),* })
        }
        Fields::Unnamed(_) => quote!(Self(#(#values),*)),
        Fields::Unit => quote!(Self),
    };
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics #core::FromContext for #name #ty_generics #where_clause {
            fn from_context(context: &#core::Context) -> ::core::result::Result<Self, #core::ContextError> {
                ::core::result::Result::Ok(#construction)
            }
        }
    })
}
