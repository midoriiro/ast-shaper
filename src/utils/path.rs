use crate::error::Error;
use crate::utils::create_ident;
use crate::utils::punctuated::PunctuatedExt;
use itertools::Itertools;
use quote::ToTokens;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::path::MAIN_SEPARATOR_STR;
use syn::punctuated::{Iter, IterMut, Punctuated};
use syn::spanned::Spanned;
use syn::token::{Comma, PathSep};
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, ParenthesizedGenericArguments, PathArguments, PathSegment, ReturnType, Token, Type, TypePath};
use crate::functions;

#[derive(Debug, Clone)]
pub struct Path {
    pub(super) segments: Punctuated<PathSegment, Token![::]>
}

impl Path {
    pub fn new(segment: impl Into<String>) -> Self {
        let ident = create_ident(&segment.into());
        let segment = PathSegment::from(ident);
        Self {
            segments: Punctuated::single(segment),
        }
    }

    pub fn join(&mut self, segment: impl Into<String>) -> &mut Self {
        let ident = create_ident(&segment.into());
        let segment = PathSegment::from(ident);
        self.segments.push(segment);
        self
    }

    fn internal_with_angle_argument(&mut self, argument: GenericArgument) -> &mut Self {
        let segment = self.segments.last_mut().unwrap();
        if segment.arguments == PathArguments::None {
            let arguments = AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: Punctuated::new(),
                gt_token: Default::default(),
            };
            segment.arguments = PathArguments::AngleBracketed(arguments);
        }
        match segment.arguments {
            PathArguments::AngleBracketed(ref mut value) => {
                value.args.push(argument);
            }
            _ => panic!("Unsupported path arguments"),
        }
        self
    }

    fn internal_with_parenthesized_argument(&mut self, argument: Type) -> &mut Self {
        let segment = self.segments.last_mut().unwrap();
        if segment.arguments == PathArguments::None {
            let arguments = ParenthesizedGenericArguments {
                paren_token: Default::default(),
                inputs: Default::default(),
                output: ReturnType::Default,
            };
            segment.arguments = PathArguments::Parenthesized(arguments);
        }
        match segment.arguments {
            PathArguments::Parenthesized(ref mut value) => {
                value.inputs.push(argument);
            }
            _ => panic!("Unsupported path arguments"),
        }
        self
    }

    pub fn with(&mut self, argument: Path) -> &mut Self {
        self.internal_with_angle_argument(GenericArgument::Type(Type::Path(TypePath {
            qself: None,
            path: argument.to_syn_path(),
        })));
        self
    }
    
    pub fn with_parameter(&mut self, argument: Path) -> &mut Self {
        self.internal_with_parenthesized_argument(Type::Path(TypePath {
            qself: None,
            path: argument.to_syn_path(),
        }));
        self
    }

    pub fn flatten(&self) -> Self {
        let mut segment = self.segments.last().unwrap().clone();
        match segment.arguments {
            PathArguments::AngleBracketed(ref mut value) => {
                for argument in value.args.iter_mut() {
                    match argument {
                        GenericArgument::Type(value) => {
                            match value {
                                Type::Path(value) => {
                                    value.path = Path::from(&value.path).flatten().to_syn_path();
                                }
                                _ => {}
                            };
                        }
                        _ => {}
                    };
                }
            }
            _ => {}
        };
        let mut segments = Punctuated::new();
        segments.push(segment.clone());
        Path {
            segments,
        }
    }

    pub fn trim_start(&mut self, prefix: &Path) -> bool {
        fn trim_path_segments(segments: &mut Punctuated<PathSegment, Token![::]>, count: usize) {
            let new_path_segments = segments.iter()
                .skip(count)
                .map(|segment| segment.clone())
                .collect::<Punctuated<PathSegment, Token![::]>>();
            *segments = new_path_segments;
        }
        fn trim_path_segment_in_generic_arguments(segment: &mut PathSegment, prefix: &Path) -> bool {
            match segment.arguments {
                PathArguments::AngleBracketed(ref mut value) => {
                    let had_been_trimmed = false;
                    for argument in value.args.iter_mut() {
                        match argument {
                            GenericArgument::Type(value) => {
                                functions::trim_path::from_type(value, prefix);
                            }
                            _ => {}
                        };
                    }
                    had_been_trimmed
                }
                _ => false
            }
        }
        let mut path_segments = prefix.segments.iter()
            .map(|segment| segment.ident.to_string())
            .collect::<VecDeque<_>>();
        let mut trim_segment_count = 1;
        for segment in self.iter_mut() {
            trim_path_segment_in_generic_arguments(segment, prefix);
            if path_segments.is_empty() {
                continue;
            }
            let segment_to_compare = path_segments.pop_front().unwrap();
            let segment_ident = segment.ident.to_string();
            if segment_ident == segment_to_compare && path_segments.is_empty() == false {
                trim_segment_count += 1;
            }
            else if segment_ident == segment_to_compare && path_segments.is_empty() {
                trim_path_segments(&mut self.segments, trim_segment_count);
                return true;
            }
        }
        false
    }

    pub fn decompose_arguments(&self, ) -> Result<Vec<Path>, Error> {
        if self.segments.is_empty() {
            return Err(Error {
                description: "Empty path".to_string(),
            })
        }
        let segment = self.segments.last().unwrap();
        match &segment.arguments {
            PathArguments::AngleBracketed(value) => {
                let arguments = value.args.iter()
                    .filter_map(|argument| {
                        match argument {
                            GenericArgument::Type(value) => {
                                match value {
                                    Type::Path(value) => Some(Path::from(value.path.clone())),
                                    _ => None
                                }
                            }
                            _ => None
                        }
                    })
                    .collect();
                Ok(arguments)
            }
            _ => Err(Error {
                description: "Unexpected arguments type".to_string(),
            })
        }
    }

    pub fn last(&self) -> Option<&PathSegment> {
        self.segments.last()
    }

    pub fn iter(&self) -> Iter<'_, PathSegment> {
        self.segments.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, PathSegment> {
        self.segments.iter_mut()
    }

    pub fn to_syn_path(&self) -> syn::Path {
        syn::Path {
            leading_colon: None,
            segments: self.segments.clone(),
        }
    }
}

impl PartialEq<Self> for Path {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

impl Eq for Path {}


impl Default for Path {
    fn default() -> Self {
        Self {
            segments: Default::default(),
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let separator = PathSep::default().to_token_stream().to_string();
        let segments = self.segments.iter()
            .map(|segment| segment.ident.to_string())
            .join(separator.as_str());
        write!(f, "{}", segments)?;
        match self.decompose_arguments() {
            Ok(value) => {
                let arguments = value.iter()
                    .join(Comma::default().to_token_stream().to_string().as_str());
                write!(f, "<{}>", arguments)
            }
            Err(_) => Ok(())
        }
    }
}

impl From<&std::path::Path> for Path {
    fn from(value: &std::path::Path) -> Self {
        let mut path = Path::default();
        value
            .to_str()
            .unwrap()
            .split(MAIN_SEPARATOR_STR)
            .for_each(|segment| {
                let ident = Ident::new(segment, segment.span());
                path.segments.push(PathSegment::from(ident));
            });
        path
    }
}

impl From<syn::Path> for Path {
    fn from(value: syn::Path) -> Self {
        let mut path = Path::default();
        for segment in value.segments {
            path.segments.push(segment);
        }
        path
    }
}

impl From<&syn::Path> for Path {
    fn from(value: &syn::Path) -> Self {
        let mut path = Path::default();
        for segment in &value.segments {
            path.segments.push(segment.clone());
        }
        path
    }
}

impl From<PathSegment> for Path {
    fn from(value: PathSegment) -> Self {
        let mut path = Path::default();
        path.segments.push(value.clone());
        path
    }
}

impl From<&PathSegment> for Path {
    fn from(value: &PathSegment) -> Self {
        let mut path = Path::default();
        path.segments.push(value.clone());
        path
    }
}

impl From<Vec<PathSegment>> for Path {
    fn from(value: Vec<PathSegment>) -> Self {
        let segments = value.iter()
            .map(|segment| segment.clone())
            .collect::<Punctuated<PathSegment, Token![::]>>();
        let mut path = Path::default();
        path.segments = segments;
        path
    }
}

impl From<Vec<&PathSegment>> for Path {
    fn from(value: Vec<&PathSegment>) -> Self {
        let segments = value.iter()
            .map(|segment| (*segment).clone())
            .collect::<Punctuated<PathSegment, Token![::]>>();
        let mut path = Path::default();
        path.segments = segments;
        path
    }
}

impl From<syn::Ident> for Path {
    fn from(value: Ident) -> Self {
        let mut path = Path::default();
        path.segments.push(PathSegment::from(value));
        path
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        let mut path = Path::default();
        path.segments.push(PathSegment::from(create_ident(value)));
        path
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let path = Path::from(value.to_string());
        path
    }
}