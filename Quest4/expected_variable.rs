mod edit_distance;
use edit_distance::*;

pub fn expected_variable(s: &str, exp: &str) -> Option<String> {
    if s.contains(' ') || s.contains('-') {
        return None;
    }
    let distance = edit_distance(&s.to_lowercase(), &exp.to_lowercase());
    let max_len = s.len().max(exp.len());
    let similarity = (((max_len - distance) as f64 * 100.0) / max_len as f64).round() as usize;
    if similarity > 50 {
        Some(format!("{}%", similarity))
    } else {
        None
    }
}
