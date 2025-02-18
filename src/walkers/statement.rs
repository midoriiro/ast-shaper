use crate::walkers::expr::ExprWalker;
use crate::walkers::macro_::MacroWalker;
use crate::walkers::pattern::PatternWalker;
use crate::walkers::Context;
use syn::Stmt;

pub struct StatementWalker;

impl StatementWalker {
    pub fn walk(
        statement: &mut syn::Stmt,
        context: &mut Context
    ) {
        let predicate_result = context.predict_stmt(statement);
        if predicate_result {
            return
        }
        match statement {
            Stmt::Local(value) => {
                PatternWalker::walk(&mut value.pat, context);
                if let Some(init) = value.init.as_mut() {
                    ExprWalker::walk(init.expr.as_mut(), context);
                    if let Some((_, diverge)) = init.diverge.as_mut() {
                        ExprWalker::walk(diverge.as_mut(), context);
                    }
                }
            }
            Stmt::Item(value) => {
                todo!()
            }
            Stmt::Expr(value, _) => {
                ExprWalker::walk(value, context);
            }
            Stmt::Macro(value) => {
                MacroWalker::walk(&mut value.mac, context);
            }
        }
    }
}