use crate::utils::path::Path;
use quote::ToTokens;
use std::collections::VecDeque;
use syn::UseTree;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UseItem(pub syn::ItemUse);

impl UseItem {
    pub fn start_with(&self, path_prefix: &Path) -> bool {
        fn walk(tree: &UseTree, path_segments: &mut VecDeque<String>) -> bool {
            match tree {
                UseTree::Path(value) => {
                    let path_segment = path_segments.pop_front().unwrap();
                    if value.ident.to_string() != path_segment && path_segments.is_empty() == false {
                        return false;
                    } else if value.ident.to_string() == path_segment && path_segments.is_empty() {
                        return true;
                    }
                    walk(&value.tree, path_segments)
                }
                UseTree::Name(value) => {
                    if path_segments.is_empty() {
                        return false;
                    }
                    let path_segment = path_segments.pop_front().unwrap();
                    if value.ident.to_string() == path_segment && path_segments.is_empty() {
                        return true;
                    }
                    false
                }
                _ => {
                    false
                }
            }
        }
        let mut path_segments = path_prefix.iter()
            .map(|segment| segment.ident.to_string())
            .collect::<VecDeque<_>>();
        walk(&self.0.tree, &mut path_segments)
    }

    pub fn trim_path_prefix(&mut self, path_prefix: &Path) -> bool {
        fn walk(tree: &UseTree, path_segments: &mut VecDeque<String>) -> Option<UseTree> {
            match tree {
                UseTree::Path(value) => {
                    let path_segment = path_segments.pop_front().unwrap();
                    if value.ident.to_string() != path_segment && path_segments.is_empty() == false {
                        return None;
                    } else if value.ident.to_string() == path_segment && path_segments.is_empty() {
                        return Some(*value.tree.clone());
                    }
                    walk(&value.tree, path_segments)
                }
                UseTree::Name(value) => {
                    if path_segments.is_empty() {
                        return None;
                    }
                    let path_segment = path_segments.pop_front().unwrap();
                    if value.ident.to_string() == path_segment && path_segments.is_empty() {
                        return Some(tree.clone());
                    }
                    None
                }
                _ => None,
            }
        }
        let mut path_segments = path_prefix.iter()
            .map(|segment| segment.ident.to_string())
            .collect::<VecDeque<_>>();
        let tree = match walk(&self.0.tree, &mut path_segments) {
            Some(value) => value,
            None => return false
        };
        self.0.tree = tree;
        true
    }

    pub fn to_string(&self) -> String {
        self.0.tree.to_token_stream().to_string()
    }
}