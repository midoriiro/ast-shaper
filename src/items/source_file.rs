use crate::items::item::{Item, ItemTrait};
use crate::items::module_item::ModuleItem;
use crate::utils::path::Path;
use crate::{debug, functions};
use itertools::Itertools;
use std::collections::HashMap;
use syn::Attribute;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceFile {
    pub attributes: Vec<Attribute>,
    pub modules: Vec<ModuleItem>,
}

impl SourceFile {
    pub fn new(attributes: Vec<Attribute>, modules: Vec<ModuleItem>) -> Self {
        Self {
            attributes,
            modules,
        }
    }

    fn resolve_ident_conflicts(&mut self) {
        let mut ident_duplications = HashMap::new();
        for module in &self.modules {
            for item in module.items.iter() {
                let item_ident = item.ident();
                if ident_duplications.contains_key(&item_ident) == false {
                    ident_duplications.insert(item_ident.clone(), Vec::new());
                }
                let module_names = ident_duplications.get_mut(&item_ident).unwrap();
                module_names.push(module.name().clone());
            }
        }
        let ident_duplications = ident_duplications.iter()
            .filter(|(_, module_names)| module_names.len() > 1)
            .collect::<HashMap<_, _>>();
        for (item_ident, module_names) in ident_duplications {
            for module_name in module_names {
                let module = self.find_module_by_name(module_name).unwrap();
                let item = module.find_item_by_ident(item_ident).unwrap();
                let namespaced_item_ident = item.ident().replace(module_name, "");
                let namespaced_item_ident = format!("{}{}", module_name, namespaced_item_ident);
                debug!("Renaming item ident from '{}' to '{}'", item_ident, namespaced_item_ident);
                functions::rename::from_item(
                    item,
                    item_ident,
                    &namespaced_item_ident,
                );
                module.items.iter_mut()
                    .for_each(|item| {
                        functions::rename::from_item(
                            item,
                            item_ident,
                            &namespaced_item_ident,
                        );
                    });
            }
        }
    }

    fn find_module_by_name(&mut self, name: &str) -> Option<&mut ModuleItem> {
        self.modules.iter_mut()
            .find(|module| module.name == name)
    }

    pub fn remove_use_items_starting_with(&mut self, prefix: &Path) {
        for module in self.modules.iter_mut() {
            module.remove_use_items_starting_with(prefix);
        }
    }

    pub fn merge(&mut self) {
        self.resolve_ident_conflicts();
        let mut module_result = ModuleItem {
            name: "".to_string(),
            file_name: "".to_string(),
            extern_crate_items: Vec::new(),
            use_items: Vec::new(),
            items: Vec::new(),
        };
        let mut use_reexport_path_prefixes = self.modules.iter()
            .map(|module| {
                let mut path = Path::new("self");
                path.join(module.file_name.clone());
                path
            })
            .collect::<Vec<_>>();
        for module in self.modules.iter_mut() {
            module_result.extern_crate_items.append(&mut module.extern_crate_items);
            module_result.use_items.append(&mut module.use_items);
            module_result.items.append(&mut module.items);
        }
        module_result.extern_crate_items = module_result.extern_crate_items.iter_mut()
            .unique_by(|item| item.ident.to_string())
            .map(|item| item.clone())
            .collect::<Vec<_>>();
        module_result.use_items = module_result.use_items.iter_mut()
            .unique_by(|item| item.to_string())
            .map(|item| item.clone())
            .collect::<Vec<_>>();
        for path_prefix in use_reexport_path_prefixes.iter_mut() {
            module_result.use_items.retain_mut(|item| {
                match item.start_with(path_prefix) {
                    true => false,
                    false => true,
                }
            });
        }
        module_result.items = module_result.items.iter()
            .sorted_by(|a, b| {
                let a = match a {
                    Item::Other(value) => {
                        value.is_fn()
                    }
                    _ => false
                };
                let b = match b {
                    Item::Other(value) => {
                        value.is_fn()
                    }
                    _ => false
                };
                b.cmp(&a) // fn first
            })
            .map(|item| item.clone())
            .collect();
        self.modules = vec![module_result]
    }
}