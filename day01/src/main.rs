use itertools::Itertools;

fn main() {
  let lines = input();

  find_entries_2(&lines)
    .iter()
    .for_each(|(x, y)| println!("{}", x * y));

  find_entries_3(&lines)
    .iter()
    .for_each(|(x, y, z)| println!("{}", x * y * z));
}

fn input() -> Vec<i32> {
  let file = std::fs::read_to_string("./input.txt")
    .expect("Something went wrong reading the file");

  let mut lines: Vec<i32> = vec![];
  for line in file.lines() {
    let parsed = line.parse().expect("Expected a number");
    lines.push(parsed);
  }
  lines
}

fn find_entries_2(xs: &[i32]) -> Vec<(i32, i32)> {
  xs
    .iter()
    .tuple_combinations()
    .filter(|&(x, y)| x + y == 2020)
    .map(|(&x, &y)| (x, y))
    .collect()
}

fn find_entries_3(xs: &[i32]) -> Vec<(i32, i32, i32)> {
  xs
    .iter()
    .tuple_combinations()
    .filter(|&(x, y, z)| x + y + z == 2020)
    .map(|(&x, &y, &z)| (x, y, z))
    .collect()
}
