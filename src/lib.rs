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
