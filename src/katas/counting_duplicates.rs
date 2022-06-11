#[cfg(test)]
mod counting_duplicates_test {
    use itertools::Itertools;
    use std::collections::HashSet;

    struct TestData {
        input: String,
        expected_output: u32,
    }

    fn counting_duplicates(text: &str) -> u32 {
        text.to_lowercase()
            .chars()
            .counts()
            .values()
            .filter(|&&i| i > 1)
            .count() as u32
    }

    fn counting_duplicates_naive_impl(text: &str) -> u32 {
        let lower_text = text.to_lowercase();

        lower_text
            .chars()
            .filter(|char| lower_text.find(*char).unwrap() != lower_text.rfind(*char).unwrap())
            .collect::<HashSet<char>>()
            .iter()
            .count() as u32
    }

    fn get_test_data() -> Vec<TestData> {
        vec![
            TestData {
                input: "abcde".to_string(),
                expected_output: 0,
            },
            TestData {
                input: "abcdea".to_string(),
                expected_output: 1,
            },
            TestData {
                input: "indivisibility".to_string(),
                expected_output: 1,
            },
        ]
    }

    #[test]
    fn counting_duplicates_test() {
        for data in get_test_data().iter() {
            assert_eq!(counting_duplicates(&data.input), data.expected_output);
        }
    }

    #[test]
    fn counting_duplicates_naive_impl_test() {
        for data in get_test_data().iter() {
            assert_eq!(
                counting_duplicates_naive_impl(&data.input),
                data.expected_output
            );
        }
    }
}
