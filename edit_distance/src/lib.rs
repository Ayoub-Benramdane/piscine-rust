pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    let mut len_source = source_chars.len();
    let len_target = target_chars.len();
    let mut del: usize;
    let mut source_or_target = "source";
    let mut min: usize;
    let mut count = 0;
    if len_source > len_target {
        min = len_target;
        del = len_source - len_target;
    } else {
        min = len_source;
        del = len_target - len_source;
        source_or_target = "target";
    }

    let mut i = 0;
    while i < len_source {
        for j in 0..len_target {
            if i == j {
                if source_chars[i] != target_chars[j] {
                    let mut next = false;
                    if i + 1 < min {
                        if source_chars[i + 1] == target_chars[j + 1] {
                            next = true;
                        }
                    }
                    if !next && del > 0 && source_or_target == "source" {
                        del -= 1;
                        source_chars.remove(i);
                        len_source -= 1;
                    } else if !next && del > 0 && source_or_target == "target" {
                        min += 1;
                        del -= 1;
                        source_chars.insert(i, target_chars[j]);
                        len_source += 1;
                    } else {
                        let _ = std::mem::replace(&mut source_chars[i], target_chars[j]);
                    }
                    count += 1;
                }
            } else if j > i {
                break;
            }
        }

        if del != 0 && i == len_source - 1 {
            del -= 1;
            source_chars.insert(len_source, '1');
            len_source += 1;
            i = 0;
            continue;
        }
        i += 1;
    }
    count
}
