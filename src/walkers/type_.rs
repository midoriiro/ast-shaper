use crate::walkers::expr::ExprWalker;
use crate::walkers::generic_param::GenericParamWalker;
use crate::walkers::macro_::MacroWalker;
use crate::walkers::path::PathWalker;
use crate::walkers::type_param_bound::TypeParamBoundWalker;
use crate::walkers::Context;
use syn::{ReturnType, Type};

pub struct TypeWalker;

impl TypeWalker {
    pub fn walk(
        type_: &mut syn::Type,
        context: &mut Context
    ) {
        let predicate_result = context.predict_type(type_);
        if predicate_result {
            return
        }
        match type_ {
            Type::Array(value) => {
                Self::walk(&mut *value.elem, context);
                ExprWalker::walk(&mut value.len, context);
            }
            Type::BareFn(value) => {
                if let Some(ref mut lifetime) = value.lifetimes {
                    for lifetime in lifetime.lifetimes.iter_mut() {
                        GenericParamWalker::walk(lifetime, context);
                    }
                }
                if let Some(ref mut variadic) = value.variadic {
                    if let Some((name, _)) = &mut variadic.name {
                        context.predict_ident(name);
                    }
                }
                for argument in value.inputs.iter_mut() {
                    if let Some((name, _)) = &mut argument.name {
                        context.predict_ident(name);
                    }
                    Self::walk(&mut argument.ty, context);
                }
                match value.output {
                    ReturnType::Type(_, ref mut value) => {
                        Self::walk(value.as_mut(), context);
                    }
                    _ => {}
                }
            }
            Type::Group(value) => {
                Self::walk(&mut value.elem, context);
            }
            Type::ImplTrait(value) => {
                for bound in value.bounds.iter_mut() {
                    TypeParamBoundWalker::walk(bound, context);
                }
            }
            Type::Macro(value) => {
                MacroWalker::walk(&mut value.mac, context);
            }
            Type::Paren(value) => {
                Self::walk(value.elem.as_mut(), context);
            }
            Type::Path(value) => {
                if let Some(ref mut qself) = value.qself {
                    Self::walk(&mut qself.ty, context);
                }
                PathWalker::walk(&mut value.path, context);
            }
            Type::Ptr(value) => {
                Self::walk(value.elem.as_mut(), context);
            }
            Type::Reference(value) => {
                Self::walk(value.elem.as_mut(), context);
                if let Some(lifetime) = value.lifetime.as_mut() {
                    context.predict_ident(&mut lifetime.ident);
                }
            }
            Type::Slice(value) => {
                TypeWalker::walk(value.elem.as_mut(), context);
            }
            Type::TraitObject(value) => {
                for bound in value.bounds.iter_mut() {
                    TypeParamBoundWalker::walk(bound, context);
                }
            }
            Type::Tuple(value) => {
                for element in value.elems.iter_mut() {
                    Self::walk(element, context);
                }
            }
            _ => {}
        }
    }
}