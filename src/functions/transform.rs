use crate::items::item::{Item, ItemTrait};
use crate::walkers::expr::ExprWalker;
use crate::walkers::statement::StatementWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;

pub fn create_context<P>(predicate: P) -> Context
where
    P: FnMut(&mut syn::Expr) -> bool + 'static
{
    Context {
        type_predicate: None,
        expr_predicate: Some(Box::new(predicate)),
        stmt_predicate: None,
        path_predicate: None,
        ident_predicate: None,
    }
}

pub fn from_item(item: &mut Item, context: &mut Context) {
    item.walk(context);
}

pub fn from_type(type_: &mut syn::Type, context: &mut Context) {
    TypeWalker::walk(type_, context)
}

pub fn from_expr(expr: &mut syn::Expr, context: &mut Context) {
    ExprWalker::walk(expr, context)
}

pub fn from_stmt(stmt: &mut syn::Stmt, context: &mut Context) {
    StatementWalker::walk(stmt, context)
}