use crate::walkers::macro_::MacroWalker;
use crate::walkers::path::PathWalker;
use crate::walkers::pattern::PatternWalker;
use crate::walkers::statement::StatementWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;
use syn::{Expr, Member, ReturnType};

pub struct ExprWalker;

impl ExprWalker {
    pub fn walk(
        expr: &mut syn::Expr,
        context: &mut Context
    ) {
        let predicate_result = context.predict_expr(expr);
        if predicate_result {
            return
        }
        match expr {
            Expr::Array(value) => {
                for element in value.elems.iter_mut() {
                    Self::walk(element, context);
                }
            }
            Expr::Assign(value) => {
                Self::walk(value.left.as_mut(), context);
                Self::walk(value.right.as_mut(), context);
            }
            Expr::Async(value) => {
                for statement in value.block.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
            }
            Expr::Await(value) => {
                Self::walk(value.base.as_mut(), context);
            }
            Expr::Binary(value) => {
                Self::walk(value.left.as_mut(), context);
                Self::walk(value.right.as_mut(), context);
            }
            Expr::Block(value) => {
                for statement in value.block.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
                if let Some(ref mut label) = value.label {
                    context.predict_ident(&mut label.name.ident);
                }
            }
            Expr::Break(value) => {
                if let Some(ref mut label) = value.label {
                    context.predict_ident(&mut label.ident);
                }
                if let Some(expr) = value.expr.as_mut() {
                    Self::walk(expr, context);
                }
            }
            Expr::Call(value) => {
                for argument in value.args.iter_mut() {
                    Self::walk(argument, context);
                }
                Self::walk(value.func.as_mut(), context);
            }
            Expr::Cast(value) => {
                TypeWalker::walk(value.ty.as_mut(), context);
                Self::walk(value.expr.as_mut(), context);
            }
            Expr::Closure(value) => {
                Self::walk(value.body.as_mut(), context);
                for pattern in value.inputs.iter_mut() {
                    PatternWalker::walk(pattern, context);
                }
                match value.output {
                    ReturnType::Default => {}
                    ReturnType::Type(_, ref mut value) => {
                        TypeWalker::walk(value.as_mut(), context);
                    }
                }
            }
            Expr::Const(value) => {
                for statement in value.block.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
            }
            Expr::Continue(value) => {
                if let Some(ref mut label) = value.label {
                    context.predict_ident(&mut label.ident);
                }
            }
            Expr::Field(value) => {
                Self::walk(value.base.as_mut(), context);
                match value.member {
                    Member::Named(ref mut value) => {
                        context.predict_ident(value);
                    }
                    _ => {}
                }
            }
            Expr::ForLoop(value) => {
                if let Some(ref mut label) = value.label {
                    context.predict_ident(&mut label.name.ident);
                }
                PatternWalker::walk(value.pat.as_mut(), context);
                Self::walk(value.expr.as_mut(), context);
                for statement in value.body.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
            }
            Expr::Group(value) => {
                Self::walk(value.expr.as_mut(), context);
            }
            Expr::If(value) => {
                Self::walk(value.cond.as_mut(), context);
                for statement in value.then_branch.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
                if let Some((_, else_branch)) = value.else_branch.as_mut() {
                    Self::walk(else_branch.as_mut(), context);
                }
            }
            Expr::Index(value) => {
                Self::walk(value.index.as_mut(), context);
                Self::walk(value.expr.as_mut(), context);
            }
            Expr::Let(value) => {
                PatternWalker::walk(value.pat.as_mut(), context);
                Self::walk(value.expr.as_mut(), context);
            }
            Expr::Loop(value) => {
                if let Some(ref mut label) = value.label {
                    context.predict_ident(&mut label.name.ident);
                }
                for statement in value.body.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
            }
            Expr::Macro(value) => {
                MacroWalker::walk(&mut value.mac, context);
            }
            Expr::Match(value) => {
                Self::walk(value.expr.as_mut(), context);
                for arm in value.arms.iter_mut() {
                    PatternWalker::walk(&mut arm.pat, context);
                    Self::walk(arm.body.as_mut(), context);
                    if let Some((_, guard)) = arm.guard.as_mut() {
                        Self::walk(guard.as_mut(), context);
                    }
                }
            }
            Expr::MethodCall(value) => {
                context.predict_ident(&mut value.method);
                Self::walk(value.receiver.as_mut(), context);
                for argument in value.args.iter_mut() {
                    Self::walk(argument, context);
                }
                if let Some(ref mut argument) = value.turbofish {
                    for argument in argument.args.iter_mut() {

                    }
                }
            }
            Expr::Paren(value) => {
                Self::walk(value.expr.as_mut(), context);
            }
            Expr::Path(value) => {
                if let Some(ref mut qself) = value.qself {
                    TypeWalker::walk(qself.ty.as_mut(), context);
                }
                PathWalker::walk(&mut value.path, context);
            }
            Expr::Range(value) => {
                if let Some(start) = value.start.as_mut() {
                    Self::walk(start.as_mut(), context);
                }
                if let Some(end) = value.end.as_mut() {
                    Self::walk(end.as_mut(), context);
                }
            }
            Expr::RawAddr(value) => {
                Self::walk(value.expr.as_mut(), context);
            }
            Expr::Reference(value) => {
                Self::walk(value.expr.as_mut(), context);
            }
            Expr::Repeat(value) => {
                Self::walk(value.expr.as_mut(), context);
                Self::walk(value.len.as_mut(), context);
            }
            Expr::Return(value) => {
                if let Some(expr) = value.expr.as_mut() {
                    Self::walk(expr, context);
                }
            }
            Expr::Struct(value) => {
                if let Some(ref mut qself) = value.qself {
                    TypeWalker::walk(qself.ty.as_mut(), context);
                }
                PathWalker::walk(&mut value.path, context);
                for field in value.fields.iter_mut() {
                    Self::walk(&mut field.expr, context);
                }
                if let Some(rest) = value.rest.as_mut() {
                    Self::walk(rest, context);
                }
            }
            Expr::Try(value) => {
                Self::walk(value.expr.as_mut(), context);
            }
            Expr::TryBlock(value) => {
                for statement in value.block.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
            }
            Expr::Tuple(value) => {
                for element in value.elems.iter_mut() {
                    Self::walk(element, context);
                }
            }
            Expr::Unary(value) => {
                Self::walk(value.expr.as_mut(), context);
            }
            Expr::Unsafe(value) => {
                for statement in value.block.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
            }
            Expr::While(value) => {
                if let Some(ref mut label) = value.label {
                    context.predict_ident(&mut label.name.ident);
                }
                Self::walk(value.cond.as_mut(), context);
                for statement in value.body.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
            }
            Expr::Yield(value) => {
                if let Some(expr) = value.expr.as_mut() {
                    Self::walk(expr.as_mut(), context);
                }
            }
            _ => {}
        }
    }
}