use crate::walkers::expr::ExprWalker;
use crate::walkers::path::PathWalker;
use crate::walkers::Context;
use quote::ToTokens;
use syn::parse2;
use crate::items::macro_::MacroInvocationArguments;

pub struct MacroWalker;

impl MacroWalker {
    pub fn walk(
        macro_: &mut syn::Macro,
        context: &mut Context
    ) {
        PathWalker::walk(&mut macro_.path, context);
        let mut arguments = parse2::<MacroInvocationArguments>(macro_.tokens.clone())
            .unwrap()
            .arguments;
        for argument in arguments.iter_mut() {
            ExprWalker::walk(argument, context);
        }
        macro_.tokens = arguments.to_token_stream();
    }
}