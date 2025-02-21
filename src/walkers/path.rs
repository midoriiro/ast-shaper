use crate::walkers::angle_bracketed_generic_arguments::AngleBracketedGenericArgumentsWalker;
use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;
use syn::{PathArguments, ReturnType};

pub struct PathWalker;

impl PathWalker {
    pub fn walk(
        path: &mut syn::Path,
        context: &mut Context
    ) {
        if context.path_predicate.is_some() {
            context.predict_path(path);
            return;
        }        
        fn walk_in_generic_arguments(path_segment: &mut syn::PathSegment, context: &mut Context) {
            match path_segment.arguments {
                PathArguments::AngleBracketed(ref mut value) => {
                    AngleBracketedGenericArgumentsWalker::walk(value, context);
                }
                PathArguments::Parenthesized(ref mut value) => {
                    match value.output {
                        ReturnType::Type(_, ref mut value) => {
                            TypeWalker::walk(value.as_mut(), context);
                        }
                        _ => {}
                    }
                    for argument in value.inputs.iter_mut() {
                        TypeWalker::walk(argument, context);
                    }
                }
                _ => {}
            }
        }
        for segment in path.segments.iter_mut() {
            walk_in_generic_arguments(segment, context);
            let mut segment_as_path = syn::Path::from(segment.clone());
            let predict_result = context.predict_path(&mut segment_as_path);
            if predict_result == false {
                continue;
            }
            *segment = segment_as_path.segments.first().unwrap().clone();
        }
    }
}