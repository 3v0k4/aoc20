use std::convert::TryInto;

#[derive(Debug, Clone)]
struct Line {
  first: i32,
  second: i32,
  c: char,
  password: String
}

fn main() {
  let lines1: Vec<Line> = parsed_input()
    .iter()
    .filter(is_valid_1)
    .cloned()
    .collect();

  let lines2: Vec<Line> = parsed_input()
    .iter()
    .filter(is_valid_2)
    .cloned()
    .collect();

  println!("{:?}", lines1.len());
  println!("{:?}", lines2.len());
}

fn parsed_input() -> Vec<Line> {
  std::fs::read_to_string("./input.txt")
    .expect("Something went wrong reading the file")
    .lines()
    .map(parse_line)
    .collect()
}

fn parse_line(line: &str) -> Line {
  let policy_password: Vec<&str> = line.split(": ").collect();
  let policy = policy_password[0];
  let rule_char: Vec<&str> = policy.split(" ").collect();
  let rule = rule_char[0];
  let min_max: Vec<&str> = rule.split("-").collect();
  let first = min_max[0].parse().expect("Expected number");
  let second = min_max[1].parse().expect("Expected number");
  let c = rule_char[1].parse().expect("Expected char");
  let password = policy_password[1].to_string();
  Line { first, second, c, password }
}

fn is_valid_1 (line: &&Line) -> bool {
  let count = line.password.chars().filter(|c| *c == line.c).collect::<Vec<_>>().len();
  count >= line.first.try_into().unwrap() && count <= line.second.try_into().unwrap()
}

fn is_valid_2 (line: &&Line) -> bool {
  let c1 = line.password.chars().nth((line.first - 1).try_into().unwrap()).unwrap();
  let c2 = line.password.chars().nth((line.second - 1).try_into().unwrap()).unwrap();
  c1 != c2 && (c1 == line.c || c2 == line.c)
}
