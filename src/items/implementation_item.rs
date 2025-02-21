use crate::items::fn_item::{FnItem, FnType};
use crate::items::item::ItemTrait;
use crate::walkers::expr::ExprWalker;
use crate::walkers::generics::GenericsWalker;
use crate::walkers::macro_::MacroWalker;
use crate::walkers::path::PathWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;
use quote::ToTokens;
use syn::{ImplItem, ItemImpl, Type};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ImplementationItem {
    pub item: ItemImpl,
    pub functions: Vec<FnItem>,
}

impl ImplementationItem {
    pub fn new(item: ItemImpl) -> Self {
        let functions = item.items.iter()
            .filter_map(|item| {
                match item {
                    ImplItem::Fn(value) => Some(FnItem::new(
                        FnType::Implementation(value.clone()),
                    )),
                    _ => None
                }
            })
            .collect();
        Self {
            item,
            functions,
        }
    }
}

impl ItemTrait for ImplementationItem {
    fn ident(&self) -> String {
        match self.item.self_ty.as_ref() {
            Type::Path(value) => {
                value.path.segments.last().as_ref().unwrap().ident.to_string()
            }
            _ => panic!(
                "Unsupported type: {:?}",
                self.item.self_ty.as_ref().to_token_stream().to_string()
            ),
        }
    }

    fn walk(&mut self, context: &mut Context) {
        GenericsWalker::walk(&mut self.item.generics, context);
        if let Some((_, ref mut path, _)) = self.item.trait_ {
            PathWalker::walk(path, context);
        }
        TypeWalker::walk(self.item.self_ty.as_mut(), context);
        for item in self.item.items.iter_mut() {
            match item {
                ImplItem::Const(value) => {
                    context.predict_ident(&mut value.ident);
                    GenericsWalker::walk(&mut value.generics, context);
                    TypeWalker::walk(&mut value.ty, context);
                    ExprWalker::walk(&mut value.expr, context);
                }
                ImplItem::Type(value) => {
                    context.predict_ident(&mut value.ident);
                    GenericsWalker::walk(&mut value.generics, context);
                    TypeWalker::walk(&mut value.ty, context);
                }
                ImplItem::Macro(value) => {
                    MacroWalker::walk(&mut value.mac, context);
                }
                _ => {}
            }
        }
        for function in self.functions.iter_mut() {
            function.walk(context);
        }
    }
}