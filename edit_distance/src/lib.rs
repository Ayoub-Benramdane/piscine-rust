use std::mem;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    let len_source = source_chars.len();
    let len_target = target_chars.len();
    let mut  del = 0;
    let mut source_or_target = "";
    let mut count = 0;
    if len_source > len_target {
        del = len_source - len_target;
        source_or_target = "source"
    } else {
        del = len_target -len_source;
        source_or_target = "target"
    }

    for i in 0..len_source {
        for j in 0..len_target {
            if i == j {
                if source_chars[i] != target_chars[j] {
                    if del > 0  && source_or_target == "source" {
                        del -= 1;
                        source_chars.remove(i);
                    } else if del > 0 && source_or_target == "target" {
                        del -= 1;
                        source_chars.insert(i, target_chars[j]);
                    } else {
                        let _ = mem::replace(&mut source_chars[i], target_chars[j]);
                    }
                    count += 1;
                }
            } else if j > i {
                break
            }
        }
    }
    count
}
