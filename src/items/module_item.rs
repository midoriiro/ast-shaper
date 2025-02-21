use crate::items::enum_item::EnumItem;
use crate::items::fn_item::{FnItem, FnType};
use crate::items::implementation_item::ImplementationItem;
use crate::items::item::{Item, ItemTrait};
use crate::items::other_item::OtherItem;
use crate::items::struct_item::StructItem;
use crate::items::use_item::UseItem;
use crate::utils::name_conventions::NamingConventions;
use crate::utils::path::Path;
use crate::walkers::Context;
use syn::{ItemExternCrate, ItemImpl, ItemUse, Type};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModuleItem {
    pub name: String,
    pub file_name: String,
    pub extern_crate_items: Vec<ItemExternCrate>,
    pub use_items: Vec<UseItem>,
    pub items: Vec<Item>,
}

impl ModuleItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: NamingConventions::to_camel_case(name),
            file_name: name.to_string(),
            extern_crate_items: Vec::new(),
            use_items: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    fn item_impl_ident(item: &ItemImpl) -> Option<String> {
        match *item.self_ty {
            Type::Path(ref value) => {
                value.path.segments.first().map(|segment| segment.ident.to_string())
            }
            _ => None
        }
    }

    pub fn push_extern_crate_item(&mut self, item: ItemExternCrate) {
        self.extern_crate_items.push(item);
    }

    pub fn push_use_item(&mut self, item: ItemUse) {
        self.use_items.push(UseItem(item));
    }

    pub fn push_struct(&mut self, item: syn::ItemStruct) {
        self.items.push(Item::Struct(StructItem::new(
            item,
            Vec::new(),
        )))
    }

    pub fn push_struct_item(&mut self, item: StructItem) {
        self.items.push(Item::Struct(item));
    }

    pub fn push_enum(&mut self, item: syn::ItemEnum) {
        self.items.push(Item::Enum(EnumItem::new(
            item,
            Vec::new(),
        )))
    }

    pub fn push_global_function(&mut self, item: syn::ItemFn) {
        self.items.push(Item::Fn(FnItem::new(FnType::Global(item))));
    }

    pub fn push_item(&mut self, item: syn::Item) {
        let item = OtherItem::new(item);
        self.items.push(Item::Other(item));
    }

    pub fn push_impl_to_struct(&mut self, item: &ItemImpl) {
        let impl_ident = Self::item_impl_ident(item).unwrap();
        let items = self.items.iter_mut()
            .filter_map(|item| {
                match item {
                    Item::Struct(value) => Some(value),
                    _ => None
                }
            })
            .find(|items| {
                items.ident() == impl_ident
            });
        let items = match items {
            Some(value) => value,
            None => return,
        };
        items.impl_items.push(ImplementationItem::new(item.clone()));
    }

    pub fn push_impl_to_enum(&mut self, item: &ItemImpl) {
        let impl_ident = Self::item_impl_ident(item).unwrap();
        let items = self.items.iter_mut()
            .filter_map(|item| {
                match item {
                    Item::Enum(value) => Some(value),
                    _ => None
                }
            })
            .find(|items| {
                items.ident() == impl_ident
            });
        let items = match items {
            Some(value) => value,
            None => return,
        };
        items.impl_items.push(ImplementationItem::new(item.clone()));
    }

    pub fn find_item_by_ident(&mut self, ident: &str) -> Option<&mut Item> {
        self.items.iter_mut()
            .find(|item| item.ident() == ident)
    }

    pub fn find_items_by(&self, mut filter: impl FnMut(&&Item) -> bool) -> Vec<&Item> {
        self.items.iter()
            .filter(|item| filter(item))
            .collect()
    }

    pub fn find_item_by(&self, mut filter: impl FnMut(&&Item) -> bool) -> Option<&Item> {
        self.items.iter()
            .find(|item| filter(item))
    }

    pub fn find_item_mut_by(&mut self, mut filter: impl FnMut(&&mut Item) -> bool) -> Option<&mut Item> {
        self.items.iter_mut()
            .find(|item| filter(item))
    }

    pub fn take_items_by(&mut self, filter: impl FnMut(&&Item) -> bool) -> Vec<Item> {
        let items = self.items.clone();
        let (take, keep): (Vec<&Item>, Vec<&Item>) = items.iter().partition(filter);
        self.items = keep.iter()
            .map(|item| (*item).clone())
            .collect::<Vec<_>>();
        take.iter()
            .map(|item| (*item).clone())
            .collect::<Vec<_>>()
    }

    fn decompose_extern_crate_items(&self) -> Vec<syn::Item> {
        self.extern_crate_items.iter()
            .map(|item| syn::Item::ExternCrate(item.clone()))
            .collect()
    }

    fn decompose_use_items(&self) -> Vec<syn::Item> {
        self.use_items.iter()
            .map(|item| syn::Item::Use(item.0.clone()))
            .collect()
    }

    fn decompose_items(&self) -> Vec<syn::Item> {
        self.items.iter()
            .flat_map(|item| {
                match item {
                    Item::Struct(value) => {
                        let mut impl_items = value.impl_items.iter()
                            .map(|impl_item| syn::Item::Impl(impl_item.item.clone()))
                            .collect::<Vec<_>>();
                        let mut items = Vec::new();
                        items.push(syn::Item::Struct(value.item.clone()));
                        items.append(&mut impl_items);
                        items
                    }
                    Item::Enum(value) => {
                        let mut impl_items = value.impl_items.iter()
                            .map(|impl_item| syn::Item::Impl(impl_item.item.clone()))
                            .collect::<Vec<_>>();
                        let mut items = Vec::new();
                        items.push(syn::Item::Enum(value.item.clone()));
                        items.append(&mut impl_items);
                        items
                    }
                    Item::Fn(value) => {
                        let mut items = Vec::new();
                        match &value.item {
                            FnType::Global(value) => items.push(syn::Item::Fn(value.clone())),
                            FnType::Implementation(_) => panic!("Unexpected implementation function"),
                        };
                        items
                    }
                    Item::Other(value) => {
                        let mut items = Vec::new();
                        items.push(value.item.clone());
                        items
                    }
                }
            })
            .collect()
    }

    pub fn decompose(&self) -> Vec<syn::Item> {
        let mut items = Vec::new();
        items.append(&mut self.decompose_extern_crate_items());
        items.append(&mut self.decompose_use_items());
        items.append(&mut self.decompose_items());
        items
    }

    pub fn remove_use_items_starting_with(&mut self, path_prefix: &Path) {
        self.use_items.retain_mut(|item| {
            match item.start_with(path_prefix) {
                true => false,
                false => true,
            }
        });
    }
}

impl ItemTrait for ModuleItem {
    fn ident(&self) -> String {
        self.name.clone()
    }

    fn walk(&mut self, context: &mut Context) {
        for item in self.items.iter_mut() {
            item.walk(context);
        }
        for use_item in self.use_items.iter_mut() {
            todo!()
        }
        for extern_crate_item in self.extern_crate_items.iter_mut() {
            context.predict_ident(&mut extern_crate_item.ident);
            if let Some((_, ref mut rename)) = extern_crate_item.rename {
                context.predict_ident(rename);
            }
        }
    }
}