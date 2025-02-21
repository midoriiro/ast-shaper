use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;
use syn::GenericArgument;

pub struct AngleBracketedGenericArgumentsWalker;

impl AngleBracketedGenericArgumentsWalker {
    pub fn walk(
        arguments: &mut syn::AngleBracketedGenericArguments,
        context: &mut Context
    ) {
        for argument in arguments.args.iter_mut() {
            match argument {
                GenericArgument::Type(value) => {
                    TypeWalker::walk(value, context);
                }
                _ => {}
            }
        };
    }
}