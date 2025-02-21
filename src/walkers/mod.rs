pub mod expr;
pub mod statement;
pub mod type_;
pub mod macro_;
pub mod pattern;
pub mod generic_argument;
pub mod generic_param;
pub mod type_param_bound;
pub mod where_clause;
pub mod signature;
pub mod generics;
pub mod fields;
pub mod path;
pub mod angle_bracketed_generic_arguments;

pub struct Context {
    pub(crate) type_predicate: Option<Box<dyn FnMut(&mut syn::Type) -> bool>>,
    pub(crate) expr_predicate: Option<Box<dyn FnMut(&mut syn::Expr) -> bool>>,
    pub(crate) stmt_predicate: Option<Box<dyn FnMut(&mut syn::Stmt) -> bool>>,
    pub(crate) path_predicate: Option<Box<dyn FnMut(&mut syn::Path) -> bool>>,
    pub(crate) ident_predicate: Option<Box<dyn FnMut(&mut syn::Ident) -> bool>>,
}

impl Context {
    pub(crate) fn predict_type(&mut self, value: &mut syn::Type) -> bool {
        if self.type_predicate.is_none() {
            return false;
        }
        self.type_predicate.as_mut().unwrap()(value)
    }

    pub(crate) fn predict_expr(&mut self, value: &mut syn::Expr) -> bool {
        if self.expr_predicate.is_none() {
            return false;
        }
        self.expr_predicate.as_mut().unwrap()(value)
    }

    pub(crate) fn predict_stmt(&mut self, value: &mut syn::Stmt) -> bool {
        if self.stmt_predicate.is_none() {
            return false;
        }
        self.stmt_predicate.as_mut().unwrap()(value)
    }

    pub(crate) fn predict_path(&mut self, value: &mut syn::Path) -> bool {
        if self.path_predicate.is_none() {
            return false;
        }
        self.path_predicate.as_mut().unwrap()(value)
    }

    pub(crate) fn predict_ident(&mut self, value: &mut syn::Ident) -> bool {
        if self.ident_predicate.is_none() {
            return false;
        }
        self.ident_predicate.as_mut().unwrap()(value)
    }
}