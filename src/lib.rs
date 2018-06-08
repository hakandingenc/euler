extern crate num_integer;

pub mod problem_one {
    pub fn multiples_of_3_and_5(n: u32) -> u32 {
        (0..n)
            .into_iter()
            .filter(|el| (el % 3) == 0 || (el % 5) == 0)
            .fold(0, |acc, x| acc + x)
    }
}

pub mod problem_two {
    fn allocate_fibo(n: u32) -> Vec<u32> {
        let mut vec = vec![1, 2];
        while vec.last().unwrap() <= &n {
            let len = vec.len();
            let new_num = vec[len - 2] + vec[len - 1];
            vec.push(new_num);
        }
        vec
    }

    fn calculate_sum_even_fibo(vec: Vec<u32>) -> u32 {
        vec.iter()
            .enumerate()
            .filter(|(index, _)| index % 3 == 1)
            .map(|(_, element)| element)
            .sum()
    }

    pub fn problem_two_script(n: u32) -> u32 {
        calculate_sum_even_fibo(allocate_fibo(n))
    }
}

pub mod problem_three {
    use std::cell::Cell;

    pub fn largest_prime_divisor(n: u64) -> u64 {
        // Need to use cell since `take_while` and `filter` borrows `n` immutably
        // but `for_each` mutably
        let n = Cell::new(n);
        let mut div = 0;
        (2..=n.get())
            .take_while(|_| n.get() > 1)
            .filter(|a| n.get() % a == 0)
            .for_each(|a| {
                while n.get() % a == 0 {
                    n.set(n.get() / a);
                }
                div = a;
            });
        div
    }
}

pub mod problem_four {
    struct ThreeDigitPair((u32, u32));

    impl ThreeDigitPair {
        fn new(a: u32, b: u32) -> Option<Self> {
            if num_digits(a) == 3 && num_digits(b) == 3 {
                Some(ThreeDigitPair((a, b)))
            } else {
                None
            }
        }
    }
    impl Iterator for ThreeDigitPair {
        type Item = (u32, u32);
        fn next(&mut self) -> Option<(u32, u32)> {
            match self.0 {
                (a, b) if a == 1000 && b == 1000 => None,
                (a, b) if b == 1000 => {
                    (self.0).0 += 1;
                    (self.0).1 = 0;
                    Some((a, b))
                }
                (a, b) => {
                    (self.0).1 += 1;
                    Some((a, b))
                }
            }
        }
    }

    fn is_palindrome(n: u32) -> bool {
        let n_str = n.to_string();
        let bytes = n_str.as_bytes();
        let len = bytes.len();
        let last_index = len - 1;
        let mid_index = len / 2;
        bytes
            .into_iter()
            .enumerate()
            .take_while(|tuple| tuple.0 <= mid_index)
            .all(|tuple| tuple.1 == &bytes[last_index - tuple.0])
    }

    pub fn num_digits(n: u32) -> usize {
        if n == 0 {
            1
        } else {
            ((n as f32).log10() + 1.).floor() as usize
        }
    }

    pub fn problem_four_script() -> u32 {
        let new_pair = ThreeDigitPair::new(100, 100).unwrap();
        new_pair
            .filter_map(|pair| {
                let compt = pair.0 * pair.1;
                if is_palindrome(compt) {
                    Some(compt)
                } else {
                    None
                }
            })
            .max()
            .unwrap()
    }
}

pub mod problem_five {
    use num_integer::lcm;

    pub fn smallest_multiple(n: u32) -> u32 {
        (1..=n).fold(1, |acc, n| lcm(acc, n))
    }
}

pub mod problem_six {
    pub fn sum_square_difference(n: u32) -> u32 {
        (1..=n).sum::<u32>().pow(2) - (1..=n).map(|n| n.pow(2)).sum::<u32>()
    }
}

pub mod problem_seven {
    pub fn nth_prime(n: usize) -> u64 {
        (2..).filter(|u| is_prime(*u)).take(n).last().unwrap()
    }

    fn is_prime(n: u64) -> bool {
        !(2..n).any(|u| n % u == 0)
    }
}

pub mod problem_eigth {
    pub fn adjacent_products(num_str: String, n: usize) -> u64 {
        let chars_vec: Vec<u64> = num_str
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let len = chars_vec.len();
        (0..=len - n)
            .map(|u| chars_vec[u..u + n].into_iter().product::<u64>())
            .max()
            .unwrap()
    }
}

pub mod problem_nine {
    use problem_four::num_digits;

    struct AtMostThreeDigitPair((u32, u32));

    impl AtMostThreeDigitPair {
        fn new(a: u32, b: u32) -> Option<Self> {
            if num_digits(a) <= 3 && num_digits(b) <= 3 && a < b {
                Some(AtMostThreeDigitPair((a, b)))
            } else {
                None
            }
        }
    }
    impl Iterator for AtMostThreeDigitPair {
        type Item = (u32, u32);
        fn next(&mut self) -> Option<(u32, u32)> {
            match self.0 {
                (a, b) if a == 999 && b == 1000 => None,
                (a, b) if a + 1 == b => {
                    (self.0).0 = 1;
                    (self.0).1 += 1;
                    Some((a, b))
                }
                (a, b) => {
                    (self.0).0 += 1;
                    Some((a, b))
                }
            }
        }
    }
    pub fn pythagorean_triplet(sum: u32) -> Vec<(u32, u32)> {
        let new_pair = AtMostThreeDigitPair::new(0, 1).unwrap();
        new_pair
            .take_while(|pair| pair.1 <= sum)
            .filter_map(|pair| {
                let sqrt = ((pair.0.pow(2) + pair.1.pow(2)) as f64).sqrt();
                if sqrt.fract() == 0. && pair.0 + pair.1 + sqrt as u32 == sum {
                    Some((pair.0, pair.1))
                } else {
                    None
                }
            })
            .collect()
    }

}
