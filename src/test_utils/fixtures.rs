use crate::items::module_item::ModuleItem;
use crate::items::struct_item::StructItem;
use crate::utils::create_ident;
use crate::utils::path::Path;
use crate::utils::punctuated::PunctuatedExt;
use quote::ToTokens;
use rstest::fixture;
use syn::punctuated::Punctuated;
use syn::ExprPath;
use syn::{AttrStyle, Attribute, Expr, FieldMutability, Fields, FieldsNamed, ItemStruct, MacroDelimiter, Meta, MetaList, MetaNameValue, Token, Type, TypePath, Visibility};

#[fixture]
pub fn module() -> ModuleItem {
    ModuleItem::new("test_module")
}

#[fixture]
pub fn required_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("u32").to_syn_path(),
        }),
    }
}

#[fixture]
pub fn optional_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Option").with(Path::new("u32")).to_syn_path(),
        }),
    }
}

#[fixture]
pub fn optional_and_optional_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Option")
                .with(Path::new("Option").with(Path::new("u32")).to_owned())
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn boxed_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Box").with(Path::new("u32")).to_syn_path(),
        }),
    }
}

#[fixture]
pub fn optional_and_boxed_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Option")
                .with(Path::new("Box").with(Path::new("u32")).to_owned())
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn ref_counter_and_refcell_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Rc")
                .with(Path::new("RefCell").with(Path::new("u32")).to_owned())
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn vec_of_primitive_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Vec")
                .with(Path::new("u32"))
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn map_of_primitive_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("HashMap")
                .with(Path::new("u32"))
                .with(Path::new("u32"))
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn complex_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("ComplexType").to_syn_path(),
        }),
    }
}


#[fixture]
pub fn field_with_attribute_as_path(mut required_field: syn::Field) -> syn::Field {
    required_field.ident = Some(create_ident("field_with_attribute_as_path"));
    required_field.attrs.push(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::Path(Path::new("attribute_as_path").to_syn_path()),
    });
    required_field
}

#[fixture]
pub fn field_with_attribute_as_name_value(mut required_field: syn::Field) -> syn::Field {
    required_field.ident = Some(create_ident("field_with_attribute_as_name_value"));
    required_field.attrs.push(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::NameValue(MetaNameValue {
            path: Path::new("attribute_as_name_value").to_syn_path(),
            eq_token: Default::default(),
            value: Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: Path::new("value").to_syn_path(),
            })
        })
    });
    required_field
}

#[fixture]
pub fn field_with_attribute_as_list(mut required_field: syn::Field) -> syn::Field {
    required_field.ident = Some(create_ident("field_with_attribute_as_list"));
    required_field.attrs.push(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::List(MetaList {
            path: Path::new("attribute_as_list").to_syn_path(),
            delimiter: MacroDelimiter::Paren(Default::default()),
            tokens: <Punctuated<syn::Path, Token![,]> as ToTokens>::to_token_stream(&Punctuated::from_iter(vec![
                Path::new("value1").to_syn_path(),
                Path::new("value2").to_syn_path(),
            ]))
        }),
    });
    required_field
}

#[fixture]
pub fn struct_with_required_field(
    mut module: ModuleItem,
    required_field: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithRequiredField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(required_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_optional_field(
    mut module: ModuleItem,
    optional_field: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithOptionalField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(optional_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_optional_and_optional_field(
    mut module: ModuleItem,
    optional_and_optional_field: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithOptionalAndOptionalField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(optional_and_optional_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_boxed_field(
    mut module: ModuleItem,
    boxed_field: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithBoxedField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(boxed_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_optional_and_boxed_field(
    mut module: ModuleItem,
    optional_and_boxed_field: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithOptionalAndBoxedField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(optional_and_boxed_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_ref_counter_and_refcell_field(
    mut module: ModuleItem,
    ref_counter_and_refcell_field: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithRefCounterAndRefCellField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(ref_counter_and_refcell_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_vec_of_primitive_field(
    mut module: ModuleItem,
    vec_of_primitive_field: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithVecOfPrimitiveField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(vec_of_primitive_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_map_of_primitive_field(
    mut module: ModuleItem,
    map_of_primitive_field: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithMapOfPrimitiveField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(map_of_primitive_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_complex_field(
    mut module: ModuleItem,
    complex_field: syn::Field,
    required_field: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithComplexField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(complex_field.clone()),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: match &complex_field.ty {
                Type::Path(value) => {
                    create_ident(value.path.segments.last().unwrap().ident.to_string())
                }
                _ => panic!("Expected path type")
            },
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(required_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_field_attributes(
    mut module: ModuleItem,
    field_with_attribute_as_path: syn::Field,
    field_with_attribute_as_name_value: syn::Field,
    field_with_attribute_as_list: syn::Field
) -> ModuleItem {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithFieldAttributes"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::from_iter(vec![
                    field_with_attribute_as_path,
                    field_with_attribute_as_name_value,
                    field_with_attribute_as_list,
                ]),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}