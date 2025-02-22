use crate::context::{Context, DeriveAttrs};
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned as _;

/// An internal call to the macro.
pub struct InternalCall {
    path: syn::Path,
    name: Option<(syn::Token![,], syn::LitStr)>,
}

impl syn::parse::Parse for InternalCall {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;

        let name = if input.peek(syn::Token![,]) {
            Some((input.parse()?, input.parse()?))
        } else {
            None
        };

        Ok(Self { path, name })
    }
}

impl InternalCall {
    pub fn expand(self) -> Result<TokenStream, Vec<syn::Error>> {
        let ctx = Context::with_module(&quote!(crate));
        let attrs = DeriveAttrs::default();
        let tokens = ctx.tokens_with_module(&attrs);

        let name = match self.name {
            Some((_, name)) => quote!(#name),
            None => match self.path.segments.last() {
                Some(last) if last.arguments.is_empty() => quote!(stringify!(#last)),
                _ => {
                    return Err(vec![syn::Error::new(
                        self.path.span(),
                        "expected last component in path to be without parameters,
                        give it an explicit name instead with `, \"Type\"`",
                    )])
                }
            },
        };

        let expand_into = quote! {
            Ok(())
        };

        let generics = syn::Generics::default();
        ctx.expand_any(&self.path, &name, &expand_into, &tokens, &generics)
    }
}

/// An internal call to the macro.
pub struct Derive {
    input: syn::DeriveInput,
}

impl syn::parse::Parse for Derive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            input: input.parse()?,
        })
    }
}

impl Derive {
    pub(super) fn expand(self) -> Result<TokenStream, Vec<syn::Error>> {
        let mut ctx = Context::new();

        let attrs = match ctx.parse_derive_attrs(&self.input.attrs) {
            Some(attrs) => attrs,
            None => return Err(ctx.errors),
        };

        let tokens = ctx.tokens_with_module(&attrs);

        let generics = &self.input.generics;
        let install_with = match ctx.expand_install_with(&self.input, &tokens, &attrs, generics) {
            Some(install_with) => install_with,
            None => return Err(ctx.errors),
        };

        let name = match attrs.name {
            Some(name) => name,
            None => syn::LitStr::new(&self.input.ident.to_string(), self.input.ident.span()),
        };

        let name = &quote!(#name);
        let ident = &self.input.ident;

        ctx.expand_any(&ident, name, &install_with, &tokens, generics)
    }
}
