use crate::items::item::{Item, ItemTrait};
use crate::utils::create_ident;
use crate::utils::path::Path;
use crate::walkers::expr::ExprWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;

fn create_context(from: String, to: String) -> Context {
    let path_predicate_from = from.clone();
    let path_predicate_to = to.clone();
    let ident_predicate_from = from.clone();
    let ident_predicate_to = to.clone();
    Context {
        type_predicate: None,
        expr_predicate: None,
        stmt_predicate: None,
        path_predicate: Some(Box::new(move |value| {
            let mut path = Path::from(value.clone());
            for segment in path.iter_mut() {
                if segment.ident.to_string() != path_predicate_from {
                    continue
                }
                segment.ident = create_ident(path_predicate_to.clone());
            }
            *value = path.to_syn_path();
            return true;
        })),
        ident_predicate: Some(Box::new(move |value| {
            if value.to_string() != ident_predicate_from {
                return false;
            }
            *value = create_ident(ident_predicate_to.clone());
            return true;
        })),
    }
}

pub fn from_item(item: &mut Item, from: &String, to: &String) {
    let mut context = create_context(from.clone(), to.clone());
    item.walk(&mut context);
}

pub fn from_type(type_: &mut syn::Type, from: &String, to: &String) {
    let mut context = create_context(from.clone(), to.clone());
    TypeWalker::walk(type_, &mut context)
}

pub fn from_expr(expr: &mut syn::Expr, from: &String, to: &String) {
    let mut context = create_context(from.clone(), to.clone());
    ExprWalker::walk(expr, &mut context)
}