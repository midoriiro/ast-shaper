use crate::items::item::ItemTrait;
use crate::items::source_file::SourceFile;
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
            if value.segments.len() == 1 {
                // trimming a path with only one segment has not meaning.
                // if prefix match then path segments will be emptied.
                return false;
            }
            let mut path = Path::from(value.clone());
            let had_been_trimmed = path.trim_start(&prefix);
            if had_been_trimmed == false {
                return false;
            }
            *value = path.to_syn_path();
            return true;
        })),
        ident_predicate: None,
    }
}

pub fn from_source_file(source_file: &mut SourceFile, prefix: &Path) {
    let mut context = create_context(prefix.clone());
    for module in source_file.modules.iter_mut() {
        for item in module.items.iter_mut() {
            item.walk(&mut context);
        }
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