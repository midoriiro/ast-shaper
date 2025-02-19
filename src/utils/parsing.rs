use crate::items::module_item::ModuleItem;
use crate::items::source_file::SourceFile;
use std::fs;
use std::fs::File;
use std::io::Read;

fn parse_source_file<P: ?Sized + AsRef<std::path::Path>>(path: &P) -> SourceFile {
    let mut file = File::open(path)
        .expect("Unable to open file");
    let mut src = String::new();
    file.read_to_string(&mut src)
        .expect("Unable to read file");
    let syntax = syn::parse_file(&src)
        .expect("Unable to parse file");
    let mut module = ModuleItem::new(path.as_ref().file_stem().unwrap().to_str().unwrap());
    for item in syntax.items {
        match item {
            syn::Item::ExternCrate(value) => {
                module.push_extern_crate_item(value);
            }
            syn::Item::Mod(value) => {
                if value.content.is_some() {
                    panic!("Module block not supported");
                }
                else {
                    // Ignoring module statement (e.g. mod my_module;)
                }
            }
            syn::Item::Use(value) => {
                module.push_use_item(value);
            }
            syn::Item::Struct(value) => {
                module.push_struct(value);
            },
            syn::Item::Enum(value) => {
                module.push_enum(value);
            }
            syn::Item::Impl(value) => {
                module.push_impl_to_struct(&value);
                module.push_impl_to_enum(&value);
            }
            syn::Item::Fn(value) => {
                module.push_global_function(value);
            }
            _ => module.push_item(item),
        }
    }
    let source_file = SourceFile::new(
        syntax.attrs.clone(),
        vec![module]
    );
    source_file
}

fn unparse_source_file(source_file: &SourceFile) -> String {
    let items = source_file.modules.iter()
        .flat_map(|module| module.decompose())
        .collect();
    let output_file = syn::File {
        shebang: None,
        attrs: source_file.attributes.clone(),
        items
    };
    prettyplease::unparse(&output_file)
}

fn unparse_item(item: &syn::Item) -> String {
    let output_file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![item.clone()]
    };
    prettyplease::unparse(&output_file)
}

fn write_code<P: ?Sized + AsRef<std::path::Path>>(code: &String, path: &P) {
    fs::create_dir_all(path)
        .expect("Unable to create output directory");
    fs::write(path, code)
        .expect("Unable to write generated file");
}

fn walk_path(path: &std::path::Path, source_files: &mut Vec<SourceFile>) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if path.is_file() && file_name.ends_with(".rs") {
            let source_file = parse_source_file(&path);
            source_files.push(source_file);
        }
        else if path.is_dir() {
            walk_path(&path, source_files);
        }
    }
}

pub trait PathExt {
    fn parse(&self) -> SourceFile;
    fn unparse(&self, source_file: &SourceFile);
    fn walk(&self) -> Vec<SourceFile>;
}

impl PathExt for std::path::Path {
    fn parse(&self) -> SourceFile {
        parse_source_file(self)
    }

    fn unparse(&self, source_file: &SourceFile) {
        let code = unparse_source_file(source_file);
        write_code(&code, self);
    }

    fn walk(&self) -> Vec<SourceFile> {
        let mut source_files = Vec::new();
        walk_path(self, &mut source_files);
        source_files
    }
}

impl PathExt for std::path::PathBuf {
    fn parse(&self) -> SourceFile {
        parse_source_file(self)
    }

    fn unparse(&self, source_file: &SourceFile) {
        let code = unparse_source_file(source_file);
        write_code(&code, self);
    }

    fn walk(&self) -> Vec<SourceFile> {
        let mut source_files = Vec::new();
        walk_path(self, &mut source_files);
        source_files
    }
}