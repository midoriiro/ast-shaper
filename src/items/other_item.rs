use crate::items::item::ItemTrait;
use crate::walkers::expr::ExprWalker;
use crate::walkers::fields::FieldsNamedWalker;
use crate::walkers::generics::GenericsWalker;
use crate::walkers::macro_::MacroWalker;
use crate::walkers::signature::SignatureWalker;
use crate::walkers::statement::StatementWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::type_param_bound::TypeParamBoundWalker;
use crate::walkers::Context;
use syn::TraitItem;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OtherItem {
    pub item: syn::Item,
}

impl OtherItem {
    pub fn new(item: syn::Item) -> Self {
        Self {
            item,
        }
    }

    pub fn is_fn(&self) -> bool {
        match self.item {
            syn::Item::Fn(_) => true,
            _ => false
        }
    }
}

impl ItemTrait for OtherItem {
    fn ident(&self) -> String {
        match &self.item {
            syn::Item::Const(value) => value.ident.to_string(),
            syn::Item::Macro(value) => value.ident.as_ref().unwrap().to_string(),
            syn::Item::Static(value) => value.ident.to_string(),
            syn::Item::Trait(value) => value.ident.to_string(),
            syn::Item::TraitAlias(value) => value.ident.to_string(),
            syn::Item::Type(value) => value.ident.to_string(),
            syn::Item::Union(value) => value.ident.to_string(),
            _ => panic!("Unsupported type"),
        }
    }

    fn walk(&mut self, context: &mut Context) {
        match &mut self.item {
            syn::Item::Const(ref mut value) => {
                context.predict_ident(&mut value.ident);
                TypeWalker::walk(value.ty.as_mut(), context);
                ExprWalker::walk(value.expr.as_mut(), context);
                GenericsWalker::walk(&mut value.generics, context);
            }
            syn::Item::Macro(value) => {
                if let Some(ref mut ident) = value.ident {
                    context.predict_ident(ident);
                }
                MacroWalker::walk(&mut value.mac, context);
            }
            syn::Item::Static(value) => {
                context.predict_ident(&mut value.ident);
                ExprWalker::walk(value.expr.as_mut(), context);
                TypeWalker::walk(value.ty.as_mut(), context);
            }
            syn::Item::Trait(value) => {
                context.predict_ident(&mut value.ident);
                GenericsWalker::walk(&mut value.generics, context);
                for bound in value.supertraits.iter_mut() {
                    TypeParamBoundWalker::walk(bound, context);
                }
                if let Some(ref mut restriction) = value.restriction {
                    todo!()
                }
                for item in value.items.iter_mut() {
                    match item {
                        TraitItem::Const(value) => {
                            context.predict_ident(&mut value.ident);
                            TypeWalker::walk(&mut value.ty, context);
                            if let Some((_, ref mut default)) = value.default {
                                ExprWalker::walk(default, context);
                            }
                            GenericsWalker::walk(&mut value.generics, context);
                        }
                        TraitItem::Fn(value) => {
                            if let Some(ref mut default) = value.default {
                                for statement in default.stmts.iter_mut() {
                                    StatementWalker::walk(statement, context);
                                }
                            }
                            SignatureWalker::walk(&mut value.sig, continue);
                        }
                        TraitItem::Type(value) => {
                            context.predict_ident(&mut value.ident);
                            if let Some((_, ref mut default)) = value.default {
                                TypeWalker::walk(default, context);
                            }
                            GenericsWalker::walk(&mut value.generics, context);
                            for bound in value.bounds.iter_mut() {
                                TypeParamBoundWalker::walk(bound, context);
                            }
                        }
                        TraitItem::Macro(value) => {
                            MacroWalker::walk(&mut value.mac, context);
                        }
                        _ => {}
                    }
                }
            }
            syn::Item::TraitAlias(value) => {
                context.predict_ident(&mut value.ident);
                GenericsWalker::walk(&mut value.generics, context);
                for bound in value.bounds.iter_mut() {
                    TypeParamBoundWalker::walk(bound, context);
                }
            }
            syn::Item::Type(value) => {
                context.predict_ident(&mut value.ident);
                GenericsWalker::walk(&mut value.generics, context);
                TypeWalker::walk(value.ty.as_mut(), context);
            }
            syn::Item::Union(value) => {
                context.predict_ident(&mut value.ident);
                GenericsWalker::walk(&mut value.generics, context);
                FieldsNamedWalker::walk(&mut value.fields, context);
            }
            _ => panic!("Unsupported type"),
        }
    }
}