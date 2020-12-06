fn main() {
  let file: String = std::fs::read_to_string("./input.txt")
    .expect("Something went wrong reading the file");
  let split: Vec<_> = file.split("\n").filter(|x| x.len() == 10).collect();
  let seats = seats(split);
  let max = seats.iter().max().unwrap();
  println!("{}", max);

  let min = seats.iter().min().unwrap();
  for i in *min..*max {
    if !seats.iter().any(|s| *s == i) {
      println!("{}", i);
      break;
    };
  }
}

fn seats(css: Vec<&str>) -> Vec<u32> {
  css.iter().map(|cs| seat(cs)).collect()
}

fn seat(cs: &str) -> u32 {
  let bits: String = cs.chars().map(|c| {
    match c {
      'B' => '1',
      'F' => '0',
      'R' => '1',
      'L' => '0',
      _   => 'X'
    }
  }).collect();
  u32::from_str_radix(&bits, 2).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn seat_BFFFBBFRRR() {
    assert_eq!(seat("BFFFBBFRRR"), 567);
  }
}
