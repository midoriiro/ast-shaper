pub(crate) mod path;
pub(crate) mod statement;
pub(crate) mod punctuated;


use crate::items::source_file::SourceFile;
use crate::utils::path::Path;
use std::fs;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Ident, ItemUse, Token, UseGlob, UseGroup, UseName, UsePath, UseTree, Visibility};

#[macro_export]
macro_rules! debug {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn to_pascal_case(name: &str) -> String {
    name.split("_")
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first_char) => {
                    let first_char_upper = first_char.to_uppercase().to_string();
                    let rest_of_word = chars.as_str().to_lowercase();
                    format!("{}{}", first_char_upper, rest_of_word)
                },
                None => String::new(),
            }
        })
        .collect::<String>()
}

pub fn generate_source_code(source_file: &mut SourceFile, output_path: &std::path::Path, output_filename: &str) {
    let items = source_file.modules.iter_mut()
        .flat_map(|module| module.decompose())
        .collect();
    let output_file = syn::File {
        shebang: None,
        attrs: source_file.attributes.clone(),
        items
    };
    let code = prettyplease::unparse(&output_file);
    fs::create_dir_all(&output_path).expect("Unable to create output directory");
    let path = output_path.join(output_filename);
    fs::write(path, code)
        .expect("Unable to write generated file");
}

pub fn create_ident(ident: impl Into<String>) -> Ident {
    let ident: String = ident.into();
    match ident.starts_with("r#") {
        true => {
            let ident = ident.replace("r#", "");
            Ident::new_raw(ident.as_str(), ident.span())
        },
        false => Ident::new(ident.as_str(), ident.span())
    }
}

pub fn create_use_tree(path: &Path) -> UseTree {
    let use_name = UseName {
        ident: path.segments.last().unwrap().ident.clone(),
    };
    let mut tree = UseTree::Name(use_name);
    for index in (0..path.segments.len() - 1).rev() {
        let segment = &path.segments[index];
        tree = UseTree::Path(UsePath {
            ident: segment.ident.clone(),
            colon2_token: Default::default(),
            tree: Box::new(tree.clone()),
        })
    }
    tree
}

pub fn create_use_group_tree(path: &Path, names: &Vec<String>) -> UseTree {
    let use_group = UseGroup {
        brace_token: Default::default(),
        items: names.iter()
            .map(|name| create_use_tree(&Path::new(name)))
            .collect::<Punctuated<UseTree, Token![,]>>(),
    };
    let mut tree = UseTree::Group(use_group);
    for index in (0..path.segments.len()).rev() {
        let segment = &path.segments[index];
        tree = UseTree::Path(UsePath {
            ident: segment.ident.clone(),
            colon2_token: Default::default(),
            tree: Box::new(tree.clone()),
        })
    }
    tree
}

pub fn create_use_glob_tree(path: &Path) -> UseTree {
    let use_glob = UseGlob {
        star_token: Default::default(),
    };
    let mut tree = UseTree::Glob(use_glob);
    for index in (0..path.segments.len()).rev() {
        let segment = &path.segments[index];
        tree = UseTree::Path(UsePath {
            ident: segment.ident.clone(),
            colon2_token: Default::default(),
            tree: Box::new(tree.clone()),
        })
    }
    tree
}

pub fn create_use(path: &Path) -> ItemUse {
    ItemUse {
        attrs: vec![],
        vis: Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree: create_use_tree(&path),
        semi_token: Default::default(),
    }
}

pub fn create_use_as_glob(path: &Path) -> ItemUse {
    ItemUse {
        attrs: vec![],
        vis: Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree: create_use_glob_tree(&path),
        semi_token: Default::default(),
    }
}

pub fn create_generic_type(ty: impl Into<String>, arguments: Vec<Path>) -> Path {
    if arguments.is_empty() {
        panic!("Passed arguments is empty");
    }
    let mut path = Path::from(ty.into());
    for argument in arguments {
        path.with(argument);
    }
    path
}