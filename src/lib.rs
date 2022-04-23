use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn count_syllables(word: &str) -> u8 {
    let mut count = 0;
    let mut prev_was_vowel = false;
    for c in word.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            if !prev_was_vowel {
                count += 1;
            }
            prev_was_vowel = true;
        } else {
            prev_was_vowel = false;
        }
    }
    if word.ends_with('e') && !word.ends_with("le") {
        count -= 1;
    }
    count
}

pub fn read_dir(dir: String) -> HashMap<String, Vec<String>> {
    let paths = fs::read_dir(dir).unwrap();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for path in paths {
        let mut cur: Vec<String> = Vec::new();
        let curpath = path.unwrap().path();
        let filename = String::from(curpath.file_name().unwrap().to_str().unwrap());
        let filetype = String::from(&filename[..(filename.len()-4)]);
        if let Ok(lines) = read_lines(curpath) {
            for line in lines {
                if let Ok(text) = line {
                    if !text.is_empty() {
                        cur.push(text);
                    }
                }
            }
            map.insert(filetype, cur);
        }
    }
    return map;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use super::count_syllables;
    #[test]
    fn test_count_syllables() {
        let words = hashmap! {
            "hello" => 2,
            "world" => 1,
            "test" => 1,
            "accent" => 2,
            "academic" => 4,
            "temperature" => 4,
            "adapter" => 3,
            "after" => 2,
            "were" => 1,
            "four" => 1,
            "fore" => 1,
            "were" => 1,
            "where" => 1,
            "door" => 1,
            "hear" => 1,
            "here" => 1,
            "hear" => 1,
            "bear" => 1,
            "fair" => 1,
            "fare" => 1,
            "faire" => 1,
            "more" => 1,
            "your" => 1,
            "there" => 1,
            "floor" => 1,
        };
        words.iter().for_each(|(word, syllables)| {
            assert_eq!(count_syllables(word), *syllables, "Word [{}]", word);
        });
    }
}
