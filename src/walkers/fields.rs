use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;
use syn::Fields;

pub struct FieldsWalker;

impl FieldsWalker {
    pub fn walk(
        fields: &mut syn::Fields,
        context: &mut Context
    ) {
        match fields {
            Fields::Named(value) => {
                FieldsNamedWalker::walk(value, context);
            }
            Fields::Unnamed(value) => {
                FieldsUnnamedWalker::walk(value, context);
            }
            Fields::Unit => {}
        }
    }
}

pub struct FieldsNamedWalker;

impl FieldsNamedWalker {
    pub fn walk(
        fields: &mut syn::FieldsNamed,
        context: &mut Context
    ) {
        for field in fields.named.iter_mut() {
            if let Some(ref mut ident) = field.ident {
                context.predict_ident(ident);
            }
            TypeWalker::walk(&mut field.ty, context);
        }
    }
}

pub struct FieldsUnnamedWalker;

impl FieldsUnnamedWalker {
    pub fn walk(
        fields: &mut syn::FieldsUnnamed,
        context: &mut Context
    ) {
        for field in fields.unnamed.iter_mut() {
            if let Some(ref mut ident) = field.ident {
                context.predict_ident(ident);
            }
            TypeWalker::walk(&mut field.ty, context);
        }
    }
}