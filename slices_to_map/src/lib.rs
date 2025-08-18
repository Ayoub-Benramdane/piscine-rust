use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U>
where
    T: Eq + Hash,
{
    let len = keys.len().min(values.len());
    let mut map = HashMap::new();
    for i in 0..len {
        map.insert(&keys[i], &values[i]);
    }
    map
}