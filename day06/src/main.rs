use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
  let file: String = std::fs::read_to_string("./input.txt").unwrap();
  let solution_1 = count(&file);
  println!("{}", solution_1);
  let solution_2 = count_all(&file);
  println!("{}", solution_2);
}

fn count(responses: &str) -> usize {
    responses
        .split("\n\n")
        .map(count_group)
        .sum()
}

fn count_group(responses: &str) -> usize {
    responses
        .replace("\n", "")
        .chars()
        .collect::<HashSet<_>>()
        .len()
}

fn count_all(responses: &str) -> usize {
    responses
        .split("\n\n")
        .map(count_group_all)
        .sum()
}

fn count_group_all(responses: &str) -> usize {
    let mut tally = HashMap::new();
    responses
        .replace("\n", "")
        .chars()
        .for_each(|x| { tally.entry(x).and_modify(|x| { *x += 1 }).or_insert_with(|| 1); () });
    tally
        .iter()
        .filter(|(_, v)| **v == responses.lines().count())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(count("a\nb\nc\n\nab\nac"), 6);
    }

    #[test]
    fn test_count_group() {
        assert_eq!(count_group("a\nb\nc"), 3);
        assert_eq!(count_group("ab\nac"), 3);
    }

    #[test]
    fn test_count_group_all() {
        assert_eq!(count_group_all("a\nb\nc"), 0);
        assert_eq!(count_group_all("abc"), 3);
    }

    #[test]
    fn test_count_all() {
        assert_eq!(count_all("abc

a
b
c

ab
ac

a
a
a
a

b"), 6);
    }
}
