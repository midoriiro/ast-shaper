use crate::items::item::ItemTrait;
use crate::walkers::signature::SignatureWalker;
use crate::walkers::statement::StatementWalker;
use crate::walkers::Context;
use std::ops::{Deref, DerefMut};
use syn::{Attribute, Block, ImplItemFn, ItemFn, Signature, Token, Visibility};

pub enum FnType {
    Global(ItemFn),
    Implementation(ImplItemFn),
}

impl FnType {
    pub fn attributes(&self) -> &Vec<Attribute> {
        match self {
            FnType::Global(value) => &value.attrs,
            FnType::Implementation(value) => &value.attrs
        }
    }

    pub fn visibility(&self) -> &Visibility {
        match self {
            FnType::Global(value) => &value.vis,
            FnType::Implementation(value) => &value.vis
        }
    }

    pub fn defaultness(&self) -> &Option<Token![default]> {
        match self {
            FnType::Global(_) => panic!("Defaultness not available in global function"),
            FnType::Implementation(value) => &value.defaultness
        }
    }

    pub fn signature(&self) -> &Signature {
        match self {
            FnType::Global(value) => &value.sig,
            FnType::Implementation(value) => &value.sig
        }
    }

    pub fn signature_mut(&mut self) -> &mut Signature {
        match self {
            FnType::Global(value) => &mut value.sig,
            FnType::Implementation(value) => &mut value.sig
        }
    }

    pub fn block(&self) -> &Block {
        match self {
            FnType::Global(value) => &value.block,
            FnType::Implementation(value) => &value.block
        }
    }

    pub fn block_mut(&mut self) -> &mut Block {
        match self {
            FnType::Global(value) => &mut value.block,
            FnType::Implementation(value) => &mut value.block
        }
    }
}

impl Clone for FnType {
    fn clone(&self) -> Self {
        match self {
            FnType::Global(value) => FnType::Global(value.clone()),
            FnType::Implementation(value) => FnType::Implementation(value.clone())
        }
    }
}

pub struct FnItem {
    pub item: FnType,
}

impl FnItem {
    pub fn new(item: FnType) -> Self {
        Self {
            item,
        }
    }

    pub fn item(&self) -> &FnType {
        &self.item
    }
}

impl Deref for FnItem {
    type Target = FnType;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl DerefMut for FnItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl ItemTrait for FnItem {
    fn ident(&self) -> String {
        self.item.signature().ident.to_string()
    }

    fn walk(&mut self, context: &mut Context) {
        match &mut self.item {
            FnType::Global(ref mut value) => {
                for statement in value.block.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
                SignatureWalker::walk(&mut value.sig, context);
            }
            FnType::Implementation(ref mut value) => {
                for statement in value.block.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
                SignatureWalker::walk(&mut value.sig, context);
            }
        }
    }
}

impl Clone for FnItem {
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
        }
    }
}