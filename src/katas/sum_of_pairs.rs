#[cfg(test)]
mod sum_pairs_test {
    use std::collections::HashSet;
    use test::Bencher;

    struct TestData<'a> {
        numbers: &'a [i8],
        sum: i8,
        expected_result: Option<(i8, i8)>,
    }

    fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
        let mut required_nums: HashSet<i8> = HashSet::new();

        for i in ints.iter() {
            if required_nums.contains(&(s - i)) {
                return Some((s - i, *i));
            }

            required_nums.insert(*i);
        }

        None
    }

    #[test]
    fn sum_pairs_test() {
        let tests = [
            TestData {
                numbers: &[],
                sum: 5,
                expected_result: None,
            },
            TestData {
                numbers: &[-1, -2, -1, -3, -6, -1],
                sum: -5,
                expected_result: Some((-2, -3)),
            },
            TestData {
                numbers: &[1, 4, 8, 7, 3, 15],
                sum: 8,
                expected_result: Some((1, 7)),
            },
            TestData {
                numbers: &[1, -2, 3, 0, -6, 1],
                sum: -6,
                expected_result: Some((0, -6)),
            },
            TestData {
                numbers: &[20, -13, 40],
                sum: -7,
                expected_result: None,
            },
            TestData {
                numbers: &[1, 2, 3, 4, 1, 0],
                sum: 2,
                expected_result: Some((1, 1)),
            },
            TestData {
                numbers: &[10, 5, 2, 3, 7, 5],
                sum: 10,
                expected_result: Some((3, 7)),
            },
            TestData {
                numbers: &[4, -2, 3, 3, 4],
                sum: 8,
                expected_result: Some((4, 4)),
            },
            TestData {
                numbers: &[0, 2, 0],
                sum: 0,
                expected_result: Some((0, 0)),
            },
            TestData {
                numbers: &[5, 9, 13, -3],
                sum: 10,
                expected_result: Some((13, -3)),
            },
        ];

        for test in tests.iter() {
            assert_eq!(sum_pairs(test.numbers, test.sum), test.expected_result);
        }
    }

    #[bench]
    fn bench_sum_pairs_at_the_beginning(b: &mut Bencher) {
        let mut nums = vec![1; 10_000_000];
        nums[0] = 8;
        nums[1] = -3;

        b.iter(|| assert_eq!(sum_pairs(&nums, 5), Some((8, -3))));
    }

    #[bench]
    fn bench_sum_pairs_in_middle(b: &mut Bencher) {
        let mut nums = vec![1; 10_000_000];
        let len = nums.len();
        nums[len / 2 - 1] = 8;
        nums[len / 2] = -3;

        b.iter(|| assert_eq!(sum_pairs(&nums, 5), Some((8, -3))));
    }

    #[bench]
    fn bench_sum_pairs_at_the_end(b: &mut Bencher) {
        let mut nums = vec![1; 10_000_000];
        let len = nums.len();
        nums[len - 2] = 8;
        nums[len - 1] = -3;

        b.iter(|| assert_eq!(sum_pairs(&nums, 5), Some((8, -3))));
    }
}
