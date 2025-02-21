use crate::items::implementation_item::ImplementationItem;
use crate::items::item::ItemTrait;
use crate::walkers::fields::FieldsWalker;
use crate::walkers::generics::GenericsWalker;
use crate::walkers::Context;
use syn::{ItemImpl, ItemStruct};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructItem {
    pub item: ItemStruct,
    pub impl_items: Vec<ImplementationItem>,
}

impl StructItem {
    pub fn new(item: ItemStruct, impl_items: Vec<ItemImpl>) -> Self {
        Self {
            item,
            impl_items: impl_items.iter()
                .map(|item| ImplementationItem::new(item.clone()))
                .collect(),
        }
    }    
}

impl ItemTrait for StructItem {
    fn ident(&self) -> String {
        self.item.ident.to_string()
    }

    fn walk(&mut self, context: &mut Context) {
        context.predict_ident(&mut self.item.ident);
        GenericsWalker::walk(&mut self.item.generics, context);
        FieldsWalker::walk(&mut self.item.fields, context);
        for impl_item in self.impl_items.iter_mut() {
            impl_item.walk(context);
        }
    }
}