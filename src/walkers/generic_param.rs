use crate::walkers::type_::TypeWalker;
use crate::walkers::type_param_bound::TypeParamBoundWalker;
use crate::walkers::Context;
use syn::GenericParam;

pub struct GenericParamWalker;

impl GenericParamWalker {
    pub fn walk(
        param: &mut syn::GenericParam,
        context: &mut Context
    ) {
        match param {
            GenericParam::Lifetime(value) => {
                for bound in value.bounds.iter_mut() {
                    context.predict_ident(&mut bound.ident);
                }
                context.predict_ident(&mut value.lifetime.ident);
            }
            GenericParam::Type(value) => {
                if let Some(ref mut default) = value.default {
                    TypeWalker::walk(default, context);
                }
                context.predict_ident(&mut value.ident);
                for bound in value.bounds.iter_mut() {
                    TypeParamBoundWalker::walk(bound, context);
                }
            }
            GenericParam::Const(value) => {
                TypeWalker::walk(&mut value.ty, context);
            }
        }
    }
}