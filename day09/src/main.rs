use itertools::Itertools;
use itertools::MinMaxResult::{MinMax};

fn main() {
    let file: String = std::fs::read_to_string("./input.txt").unwrap();
    let parsed: Vec<u64> = file.lines().map(|x| x.parse().unwrap()).collect();
    let solution_1 = solution_1(&parsed, 25).unwrap();
    println!("{:?}", solution_1);
    let solution_2 = solution_2(&parsed, solution_1).unwrap();
    println!("{:?}", solution_2);
}

fn solution_1(xs: &[u64], window: usize) -> Option<u64> {
    xs
        .windows(window + 1)
        .find(|xs| !is_sum(&xs[0..window], xs[window]))
        .map(|x| x[window])
}

fn is_sum(xs: &[u64], x: u64) -> bool {
    xs
        .iter()
        .tuple_combinations()
        .map(|(y, z)| y + z)
        .map(|y| x == y)
        .any(|x| x)
}

fn solution_2(xs: &[u64], x: u64) -> Option<u64> {
    for i in 0..xs.len() {
        match contiguous(&xs[i..], x).map(|xs| xs.into_iter().minmax()) {
            Some(MinMax(x, y)) => return Some(x + y),
            _ => {},
        }
    }
    None
}

fn contiguous(xs: &[u64], x: u64) -> Option<Vec<u64>> {
    let mut sum: u64 = 0;
    for i in 0..xs.len() {
        sum += xs[i];
        if sum == x { return Some(xs[0..i].to_vec()) }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sum() {
        let x = 40;
        let xs = [35, 20, 15, 25, 47];
        assert_eq!(is_sum(&xs, x), true);

        let x = 127;
        let xs = [95, 102, 117, 150, 182];
        assert_eq!(is_sum(&xs, x), false);
    }

    #[test]
    fn test_solution_1() {
        let xs = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        assert_eq!(solution_1(&xs, 5), Some(127));
    }

    #[test]
    fn test_solution_2() {
        let xs = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        assert_eq!(solution_2(&xs, 127), Some(15 + 47));
    }
}
