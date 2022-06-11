#[cfg(test)]
mod duplicate_encoder_test {
    fn duplicate_encode(word: &str) -> String {
        let lowercase_word = word.to_lowercase();

        lowercase_word
            .chars()
            .map(|char| match lowercase_word.matches(char).count() {
                1 => "(",
                _ => ")",
            })
            .collect()
    }

    #[test]
    fn duplicate_encoder_test() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}
