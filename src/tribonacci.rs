#[cfg(test)]
mod tribonacci_test {
    #[derive(Debug)]
    struct TribonacciData {
        input: [f64; 3],
        expected_output: Vec<f64>,
    }

    fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
        if n <= 3 {
            return signature[..n].to_vec();
        }

        let mut result = signature.to_vec();

        for i in 0..n - signature.len() {
            result.push(result[i..i + 3].iter().sum());
        }

        return result;
    }

    fn generate_tribonacci_data() -> Vec<TribonacciData> {
        return vec![
            TribonacciData {
                input: [0., 1., 1.],
                expected_output: vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.],
            },
            TribonacciData {
                input: [1., 0., 0.],
                expected_output: vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.],
            },
            TribonacciData {
                input: [0., 0., 0.],
                expected_output: vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
            },
            TribonacciData {
                input: [1., 2., 3.],
                expected_output: vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.],
            },
            TribonacciData {
                input: [3., 2., 1.],
                expected_output: vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.],
            },
            TribonacciData {
                input: [1., 1., 1.],
                expected_output: vec![1.],
            },
            TribonacciData {
                input: [300., 200., 100.],
                expected_output: vec![],
            },
            TribonacciData {
                input: [0.5, 0.5, 0.5],
                expected_output: vec![
                    0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5,
                    1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5,
                    266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5,
                ],
            },
        ];
    }

    #[test]
    fn tribonacci_tests() {
        for data in generate_tribonacci_data().iter() {
            assert_eq!(
                tribonacci(&data.input, data.expected_output.len()),
                data.expected_output
            );
        }
    }
}
