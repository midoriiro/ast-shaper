use crate::walkers::type_::TypeWalker;
use crate::walkers::Context;
use syn::{GenericArgument, PathArguments, ReturnType};

pub struct PathWalker;

impl PathWalker {
    pub fn walk(
        path: &mut syn::Path,
        context: &mut Context
    ) {
        fn walk_in_generic_arguments(path_segment: &mut syn::PathSegment, context: &mut Context) {
            match path_segment.arguments {
                PathArguments::AngleBracketed(ref mut value) => {
                    for argument in value.args.iter_mut() {
                        match argument {
                            GenericArgument::Type(value) => {
                                TypeWalker::walk(value, context);
                            }
                            _ => {}
                        }
                    };
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