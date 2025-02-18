use crate::walkers::generic_param::GenericParamWalker;
use crate::walkers::path::PathWalker;
use crate::walkers::Context;
use syn::{CapturedParam, TypeParamBound};

pub struct TypeParamBoundWalker;

impl TypeParamBoundWalker {
    pub fn walk(
        param: &mut syn::TypeParamBound,
        context: &mut Context
    ) {
        match param {
            TypeParamBound::Trait(value) => {
                if let Some(lifetimes) = value.lifetimes.as_mut() {
                    for lifetime in lifetimes.lifetimes.iter_mut() {
                        GenericParamWalker::walk(lifetime, context);
                    }
                }
                PathWalker::walk(&mut value.path, context);
            }
            TypeParamBound::Lifetime(value) => {
                context.predict_ident(&mut value.ident);
            }
            TypeParamBound::PreciseCapture(value) => {
                for param in value.params.iter_mut() {
                    match param {
                        CapturedParam::Lifetime(value) => {
                            context.predict_ident(&mut value.ident);
                        }
                        CapturedParam::Ident(value) => {
                            context.predict_ident(value);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}