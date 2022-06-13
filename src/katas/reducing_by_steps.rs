#[cfg(test)]
mod reducing_by_steps_test {
    use std::cmp::max;

    fn sum(x: i64, y: i64) -> i64 {
        x + y
    }

    fn maxi(x: i64, y: i64) -> i64 {
        max(x, y)
    }

    fn gcdi(m: i64, n: i64) -> i64 {
        let (mut m, mut n) = (m, n);

        loop {
            let temp = m.checked_rem(n).unwrap_or(0);

            m = n;
            n = temp;

            if m == 0 || n == 0 {
                break;
            }
        }

        return m.abs();
    }

    fn lcmu(a: i64, b: i64) -> i64 {
        let sum = (a * b).abs();
        let gcdi = gcdi(a, b);

        sum.checked_div(gcdi).unwrap_or(0)
    }

    fn oper_array<F>(f: F, a: &[i64], init: i64) -> Vec<i64>
    where
        F: Fn(i64, i64) -> i64,
    {
        a.into_iter()
            .scan(init, |state, &x| {
                *state = f(*state, x);

                Some(*state)
            })
            .collect()
    }

    #[test]
    fn test_reduce_sum() {
        let array = [18, 69, -90, -78, 65, 40];
        let expected_vec: Vec<i64> = vec![18, 87, -3, -81, -16, 24];
        let initial = 0;

        assert_eq!(oper_array(sum, &array, initial), expected_vec);
    }

    #[test]
    fn test_reduce_lcmu() {
        let array = [18, 69, -90, -78, 65, 40];
        let expected_vec: Vec<i64> = vec![18, 414, 2070, 26910, 26910, 107640];
        let initial = array[0];

        assert_eq!(oper_array(lcmu, &array, initial), expected_vec);
    }

    #[test]
    fn text_reduce_maxi() {
        let array = [18, 69, -90, -78, 65, 40];
        let expected_vec: Vec<i64> = vec![18, 69, 69, 69, 69, 69];
        let initial = array[0];

        assert_eq!(oper_array(maxi, &array, initial), expected_vec);
    }

    #[test]
    fn test_reduce_gcdi() {
        let array = [18, 69, -90, -78, 65, 40];
        let expected_vec: Vec<i64> = vec![18, 3, 3, 3, 1, 1];
        let initial = array[0];

        assert_eq!(oper_array(gcdi, &array, initial), expected_vec);
    }
}
