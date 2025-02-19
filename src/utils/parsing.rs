use std::fs;
use std::fs::File;
use std::io::Read;
use crate::items::module_item::ModuleItem;
use crate::items::source_file::SourceFile;

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
        module.push_item(item);
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

pub fn write_code<P: ?Sized + AsRef<std::path::Path>>(code: &String, path: &P) {
    fs::create_dir_all(path)
        .expect("Unable to create output directory");
    fs::write(path, code)
        .expect("Unable to write generated file");
}

pub trait PathExt {
    fn parse(&self) -> SourceFile;
    fn unparse(&self, source_file: &SourceFile);
}

impl PathExt for std::path::Path {
    fn parse(&self) -> SourceFile {
        parse_source_file(self)
    }

    fn unparse(&self, source_file: &SourceFile) {
        let code = unparse_source_file(source_file);
        write_code(&code, self);
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
}