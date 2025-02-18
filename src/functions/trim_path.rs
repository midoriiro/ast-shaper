use crate::utils::path::Path;
use crate::walkers::expr::ExprWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;

fn create_context(prefix: Path) -> Context {
    Context {
        type_predicate: None,
        expr_predicate: None,
        stmt_predicate: None,
        path_predicate: Some(Box::new(move |value| {
            let mut path = Path::from(value.clone());
            let had_been_trimmed = path.trim_start(&prefix);
            if had_been_trimmed == false {
                return false;
            }
            *value = path.to_syn_path();
            had_been_trimmed
        })),
        ident_predicate: None,
    }
}

pub fn from_type(type_: &mut syn::Type, prefix: &Path) {
    let mut context = create_context(prefix.clone());
    TypeWalker::walk(type_, &mut context)
}

pub fn from_expr(expr: &mut syn::Expr, prefix: &Path) {
    let mut context = create_context(prefix.clone());
    ExprWalker::walk(expr, &mut context)
}