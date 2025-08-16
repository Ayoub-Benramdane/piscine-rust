pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    if prefix.is_empty() {
        return Some(s);
    }
    if s.len() < prefix.len() {
        return None;
    }
    if s[0..prefix.len()] == *prefix {
        return Some(&s[prefix.len()..]) ;
    }

    None
    
}