use crate::walkers::generic_param::GenericParamWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::type_param_bound::TypeParamBoundWalker;
use crate::walkers::Context;
use syn::WherePredicate;

pub struct WhereClauseWalker;

impl WhereClauseWalker {
    pub fn walk(
        where_clause: &mut syn::WhereClause,
        context: &mut Context
    ) {
        for predicate in where_clause.predicates.iter_mut() {
            match predicate {
                WherePredicate::Lifetime(value) => {
                    context.predict_ident(&mut value.lifetime.ident);
                    for bound in value.bounds.iter_mut() {
                        context.predict_ident(&mut bound.ident);
                    }
                }
                WherePredicate::Type(value) => {
                    TypeWalker::walk(&mut value.bounded_ty, context);
                    if let Some(ref mut lifetime) = value.lifetimes {
                        for lifetime in lifetime.lifetimes.iter_mut() {
                            GenericParamWalker::walk(lifetime, context);
                        }
                    }
                    for bound in value.bounds.iter_mut() {
                        TypeParamBoundWalker::walk(bound, context);
                    }
                }
                _ => {}
            }
        }
    }
}