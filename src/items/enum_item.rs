use crate::items::implementation_item::ImplementationItem;
use crate::items::item::ItemTrait;
use crate::walkers::expr::ExprWalker;
use crate::walkers::fields::FieldsWalker;
use crate::walkers::generics::GenericsWalker;
use crate::walkers::Context;
use syn::{ItemEnum, ItemImpl};

pub struct EnumItem {
    pub item: ItemEnum,
    pub impl_items: Vec<ImplementationItem>,
}

impl EnumItem {
    pub fn new(item: ItemEnum, impl_items: Vec<ItemImpl>) -> Self {
        Self {
            item,
            impl_items: impl_items.iter()
                .map(|item| ImplementationItem::new(item.clone()))
                .collect(),
        }
    }
}

impl ItemTrait for EnumItem {
    fn ident(&self) -> String {
        self.item.ident.to_string()
    }

    fn walk(&mut self, context: &mut Context) {
        context.predict_ident(&mut self.item.ident);
        GenericsWalker::walk(&mut self.item.generics, context);
        for variant in self.item.variants.iter_mut() {
            context.predict_ident(&mut variant.ident);
            FieldsWalker::walk(&mut variant.fields, context);
            if let Some((_, ref mut discriminant)) = variant.discriminant {
                ExprWalker::walk(discriminant, context);
            }
        }
        for impl_item in self.impl_items.iter_mut() {
            impl_item.walk(context);
        }
    }
}

impl Clone for EnumItem {
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
            impl_items: self.impl_items.clone(),
        }
    }
}