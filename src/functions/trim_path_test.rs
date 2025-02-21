use crate::utils::parsing::TokenStreamExt;
use crate::utils::path::Path;
use pretty_assertions::assert_eq;
use quote::quote;
use rstest::rstest;

#[rstest]
fn from_source_file() {
    let quote = quote! {
        use std::path::PathBuf;

        struct MyStruct {
            my_field: std::path::PathBuf
        }

        impl MyStruct {
            fn new(path: std::path::PathBuf) -> Self {
                Self {
                    myfield: path
                }
            }
        }
    };
    let path_prefix = Path::new("std").join("path").to_owned();
    let mut source_file = quote.parse();
    crate::functions::trim_path::from_source_file(
        &mut source_file,
        &path_prefix,
    );
    let expected_quote = quote! {
        use std::path::PathBuf;

        struct MyStruct {
            my_field: PathBuf
        }

        impl MyStruct {
            fn new(path: PathBuf) -> Self {
                Self {
                    myfield: path
                }
            }
        }
    };
    let expected_source_file = expected_quote.parse();
    assert_eq!(expected_source_file, source_file);
}

#[rstest]
fn with_one_segment() {
    let quote = quote! {
        use std::path::PathBuf;

        struct MyStruct {
            my_field: PathBuf
        }

        impl MyStruct {
            fn new(path: PathBuf) -> Self {
                Self {
                    myfield: path
                }
            }
        }
    };
    let path_prefix = Path::new("PathBuf");
    let mut source_file = quote.parse();
    crate::functions::trim_path::from_source_file(
        &mut source_file,
        &path_prefix,
    );
    let expected_quote = quote! {
        use std::path::PathBuf;

        struct MyStruct {
            my_field: PathBuf
        }

        impl MyStruct {
            fn new(path: PathBuf) -> Self {
                Self {
                    myfield: path
                }
            }
        }
    };
    let expected_source_file = expected_quote.parse();
    assert_eq!(expected_source_file, source_file);
}