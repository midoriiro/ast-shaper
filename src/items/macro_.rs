use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Token};

pub struct MacroInvocationArguments {
    pub arguments: Punctuated<Expr, Token![,]>,
}

impl Parse for MacroInvocationArguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MacroInvocationArguments {
            arguments: Punctuated::parse_terminated(input)?,
        })
    }
}