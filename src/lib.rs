#![allow(dead_code)]

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
            .fold(0, |acc, el| acc + el)
    }

    pub fn problem_two_script(n: u32) -> u32 {
        calculate_sum_even_fibo(allocate_fibo(n))
    }
}
