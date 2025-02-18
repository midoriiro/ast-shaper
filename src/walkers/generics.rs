use crate::walkers::generic_param::GenericParamWalker;
use crate::walkers::where_clause::WhereClauseWalker;
use crate::walkers::Context;

pub struct GenericsWalker;

impl GenericsWalker {
    pub fn walk(
        generics: &mut syn::Generics,
        context: &mut Context
    ) {
        for param in generics.params.iter_mut() {
            GenericParamWalker::walk(param, context);
        }
        if let Some(ref mut where_clause) = generics.where_clause {
            WhereClauseWalker::walk(where_clause, context);
        }
    }
}