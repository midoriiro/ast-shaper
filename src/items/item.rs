use crate::items::enum_item::EnumItem;
use crate::items::fn_item::FnItem;
use crate::items::other_item::OtherItem;
use crate::items::struct_item::StructItem;
use crate::walkers::Context;

pub trait ItemTrait {
    fn ident(&self) -> String;
    fn walk(&mut self, context: &mut Context);
}

pub enum Item {
    Struct(StructItem),
    Enum(EnumItem),
    Fn(FnItem),
    Other(OtherItem),
}

impl Item {
    pub fn to_syn_item(&self) -> syn::Item {
        match self {
            Item::Struct(value) => syn::Item::Struct(value.item.clone()),
            Item::Enum(value) => syn::Item::Enum(value.item.clone()),
            Item::Fn(value) => value.item.to_syn_item(),
            Item::Other(value) => value.item.clone(),
        }
    }

    pub fn as_struct_ref(&self) -> Option<&StructItem> {
        match self {
            Item::Struct(value) => Some(value),
            _ => None
        }
    }

    pub fn as_struct_mut(&mut self) -> Option<&mut StructItem> {
        match self {
            Item::Struct(value) => Some(value),
            _ => None
        }
    }
}

impl ItemTrait for Item {
    fn ident(&self) -> String {
        match self {
            Item::Struct(value) => value.ident(),
            Item::Enum(value) => value.ident(),
            Item::Fn(value) => value.ident(),
            Item::Other(value) => value.ident(),
        }
    }

    fn walk(&mut self, context: &mut Context) {
        match self {
            Item::Struct(value) => value.walk(context),
            Item::Enum(value) => value.walk(context),
            Item::Fn(value) => value.walk(context),
            Item::Other(value) => value.walk(context),
        }
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        match self {
            Item::Struct(value) => Item::Struct(value.clone()),
            Item::Enum(value) => Item::Enum(value.clone()),
            Item::Fn(value) => Item::Fn(value.clone()),
            Item::Other(value) => Item::Other(value.clone()),
        }
    }
}