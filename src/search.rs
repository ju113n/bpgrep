use std::{cmp::max, collections::HashMap};

type Byte = u8;
type Shift = i32;

// Boyer-Moore string search
pub fn boyer_moore(text: &String, pattern: &String) -> Vec<Shift> {
    bad_character(text.as_bytes(), pattern.as_bytes())
}

// bad character heuristic
fn bad_character(text: &[Byte], pattern: &[Byte]) -> Vec<Shift> {
    let lookup_table = lookup(pattern);
    let size_t = text.len() as Shift;
    let size_p = pattern.len() as Shift;
    let mut result: Vec<Shift> = Vec::new();

    // iterate over text
    let size_dif = size_t - size_p;
    let mut cursor: Shift = 0;
    while cursor < size_dif {
        // iterate over pattern
        let mut i = size_p - 1;
        while i >= 0 && text[(cursor + i) as usize] == pattern[i as usize] {
            i -= 1;
        }

        // pattern match
        if i == -1 {
            result.push(cursor);
            cursor += if cursor + size_p < size_t { size_p } else { 1 };
        }
        // shift
        else {
            let bad_c = &text[(cursor + i) as usize];
            cursor += if lookup_table.contains_key(bad_c) {
                max(1, i - *lookup_table.get(bad_c).unwrap())
            } else {
                i + 1
            };
        }
    }
    result
}

// preprocess pattern into lookup table
fn lookup(pattern: &[Byte]) -> HashMap<&Byte, Shift> {
    pattern
        .iter()
        .enumerate()
        .map(|(i, u)| (u, i as Shift))
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        let pattern = String::from("RPCRQ");
        let mut lookup_table: HashMap<&Byte, Shift> = HashMap::new();
        lookup_table.insert(&('R' as Byte), 3);
        lookup_table.insert(&('P' as Byte), 1);
        lookup_table.insert(&('C' as Byte), 2);
        lookup_table.insert(&('Q' as Byte), 4);

        let result = lookup(&pattern.as_bytes());

        assert_eq!(result, lookup_table);
    }

    #[test]
    fn test_bad_character() {
        let text = String::from("AYRRQMGRPCRQA");
        let pattern = String::from("RPCRQ");
        let mut found: Vec<Shift> = Vec::new();
        found.push(7);

        let result = bad_character(&text.as_bytes(), &pattern.as_bytes());

        assert_eq!(result, found);
    }
}
