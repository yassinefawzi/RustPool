use std::collections::BTreeSet;

pub fn flatten_tree<T: Clone + Ord>(tree: &BTreeSet<T>) -> Vec<T> {
	tree.iter().cloned().collect()
}