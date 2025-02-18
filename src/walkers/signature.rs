use crate::walkers::generic_param::GenericParamWalker;
use crate::walkers::pattern::PatternWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::where_clause::WhereClauseWalker;
use crate::walkers::Context;
use syn::{FnArg, ReturnType};

pub struct SignatureWalker;

impl SignatureWalker {
    pub fn walk(
        signature: &mut syn::Signature,
        context: &mut Context
    ) {
        context.predict_ident(&mut signature.ident);
        for param in signature.generics.params.iter_mut() {
            GenericParamWalker::walk(param, context);
        }
        if let Some(ref mut where_clause) = signature.generics.where_clause {
            WhereClauseWalker::walk(where_clause, context);
        }
        if let Some(ref mut variadic) = signature.variadic {
            if let Some((ref mut pattern, _)) = variadic.pat {
                PatternWalker::walk(pattern, context);
            }
        }
        for argument in signature.inputs.iter_mut() {
            match argument {
                FnArg::Receiver(value) => {
                    TypeWalker::walk(value.ty.as_mut(), context);
                    if let Some((_, ref mut reference)) = value.reference {
                        if let Some(ref mut lifetime) = reference {
                            context.predict_ident(&mut lifetime.ident);
                        }
                    }
                }
                FnArg::Typed(value) => {
                    TypeWalker::walk(value.ty.as_mut(), context);
                    PatternWalker::walk(value.pat.as_mut(), context);
                }
            }
        }
        match signature.output {
            ReturnType::Default => {}
            ReturnType::Type(_, ref mut type_) => {
                TypeWalker::walk(type_.as_mut(), context);
            }
        }
    }
}