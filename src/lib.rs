pub mod problem_one {
    pub fn multiples_of_3_and_5(n: u32) -> u32 {
        (0..n)
            .into_iter()
            .filter(|el| (el % 3) == 0 || (el % 5) == 0)
            .fold(0, |acc, x| acc + x)
    }
}
