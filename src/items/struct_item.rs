use crate::items::implementation_item::ImplementationItem;
use crate::items::item::ItemTrait;
use crate::walkers::fields::FieldsWalker;
use crate::walkers::generics::GenericsWalker;
use crate::walkers::Context;
use syn::{ItemImpl, ItemStruct};

pub struct StructItem {
    item: ItemStruct,
    impl_items: Vec<ImplementationItem>,
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

    pub fn item(&self) -> &ItemStruct {
        &self.item
    }

    pub fn item_mut(&mut self) -> &mut ItemStruct {
        &mut self.item
    }

    pub fn impl_items_mut(&mut self) -> &mut Vec<ImplementationItem> {
        &mut self.impl_items
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

impl Clone for StructItem {
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
            impl_items: self.impl_items.clone(),
        }
    }
}