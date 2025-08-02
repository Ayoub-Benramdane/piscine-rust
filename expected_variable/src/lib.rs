use edit_distance::*;
use convert_case::{Casing};

pub fn expected_variable(source: &str, target: &str) -> Option<String> {
    let source_to_lower = source.to_lowercase();
    let target_to_lower = target.to_lowercase();
    let dist = edit_distance(&source_to_lower, &target_to_lower);
    if target_to_lower != target_to_lower.to_case(convert_case::Case::Camel) && target_to_lower != target_to_lower.to_case(convert_case::Case::Snake) {
        return None
    }
    let prc = 100.0 - (dist as f64 / target.len() as f64 *100.0);
    if prc > 50.0 {
        Some(format!("{}%", prc.round()))
    } else {
        None
    }
}