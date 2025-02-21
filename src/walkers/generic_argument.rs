use crate::walkers::angle_bracketed_generic_arguments::AngleBracketedGenericArgumentsWalker;
use crate::walkers::expr::ExprWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::type_param_bound::TypeParamBoundWalker;
use crate::walkers::Context;
use syn::GenericArgument;

pub struct GenericArgumentWalker;

impl GenericArgumentWalker {
    pub fn walk(
        param: &mut syn::GenericArgument,
        context: &mut Context
    ) {
        match param {
            GenericArgument::Lifetime(value) => {
                context.predict_ident(&mut value.ident);
            }
            GenericArgument::Type(value) => {
                TypeWalker::walk(value, context);
            }
            GenericArgument::Const(value) => {
                ExprWalker::walk(value, context);
            }
            GenericArgument::AssocType(value) => {
                context.predict_ident(&mut value.ident);
                TypeWalker::walk(&mut value.ty, context);
                if let Some(generics) = value.generics.as_mut() {
                    AngleBracketedGenericArgumentsWalker::walk(generics, context);
                }
            }
            GenericArgument::AssocConst(value) => {
                context.predict_ident(&mut value.ident);
                ExprWalker::walk(&mut value.value, context);
                if let Some(generics) = value.generics.as_mut() {
                    AngleBracketedGenericArgumentsWalker::walk(generics, context);
                }
            }
            GenericArgument::Constraint(value) => {
                context.predict_ident(&mut value.ident);
                for bound in value.bounds.iter_mut() {
                    TypeParamBoundWalker::walk(bound, context);
                }
                if let Some(generics) = value.generics.as_mut() {
                    AngleBracketedGenericArgumentsWalker::walk(generics, context);
                }
            }
            _ => {}
        }
    }
}