use crate::walkers::expr::ExprWalker;
use crate::walkers::macro_::MacroWalker;
use crate::walkers::path::PathWalker;
use crate::walkers::statement::StatementWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;
use syn::{Member, Pat};

pub struct PatternWalker;

impl PatternWalker {
    pub fn walk(
        pattern: &mut syn::Pat,
        context: &mut Context
    ) {
        match pattern {
            Pat::Const(value) => {
                for statement in value.block.stmts.iter_mut() {
                    StatementWalker::walk(statement, context);
                }
            }
            Pat::Ident(value) => {
                context.predict_ident(&mut value.ident);
            }
            Pat::Macro(value) => {
                MacroWalker::walk(&mut value.mac, context);
            }
            Pat::Or(value) => {
                for case in &mut value.cases.iter_mut() {
                    Self::walk(case, context);
                }
            }
            Pat::Paren(value) => {
                Self::walk(value.pat.as_mut(), context);
            }
            Pat::Path(value) => {
                if let Some(ref mut qself) = value.qself {
                    TypeWalker::walk(qself.ty.as_mut(), context);
                }
                PathWalker::walk(&mut value.path, context);
            }
            Pat::Range(value) => {
                if let Some(ref mut start) = value.start {
                    ExprWalker::walk(start.as_mut(), context);
                }
                if let Some(ref mut end) = value.end {
                    ExprWalker::walk(end.as_mut(), context);
                }
            }
            Pat::Reference(value) => {
                Self::walk(value.pat.as_mut(), context);
            }
            Pat::Slice(value) => {
                for element in value.elems.iter_mut() {
                    Self::walk(element, context);
                }
            }
            Pat::Struct(value) => {
                if let Some(ref mut qself) = value.qself {
                    TypeWalker::walk(qself.ty.as_mut(), context);
                }
                PathWalker::walk(&mut value.path, context);
                for field in value.fields.iter_mut() {
                    Self::walk(field.pat.as_mut(), context);
                    match field.member {
                        Member::Named(ref mut value) => {
                            context.predict_ident(value);
                        }
                        _ => {}
                    }
                }
            }
            Pat::Tuple(value) => {
                for element in &mut value.elems.iter_mut() {
                    Self::walk(element, context);
                }
            }
            Pat::TupleStruct(value) => {
                if let Some(ref mut qself) = value.qself {
                    TypeWalker::walk(qself.ty.as_mut(), context);
                }
                for element in value.elems.iter_mut() {
                    Self::walk(element, context);
                }
                PathWalker::walk(&mut value.path, context);
            }
            Pat::Type(value) => {
                Self::walk(value.pat.as_mut(), context);
                TypeWalker::walk(value.ty.as_mut(), context);
            }
            _ => {}
        }
    }
}