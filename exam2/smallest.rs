use std::collections::HashMap;
pub fn smallest(h: HashMap<&str, i32>) -> i32 {
	if h.is_empty() {
		return i32::MAX;
	}
	*h.values().min().unwrap()
}
