fn main() {
  let lines = input();
  let opt_entries_2 = find_entries_2(&lines);
  if let Some((x, y)) = opt_entries_2 {
    println!("{}", x * y);
  }

  let opt_entries_3 = find_entries_3(&lines);
  if let Some((x, y, z)) = opt_entries_3 {
    println!("{}", x * y * z);
  }
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

fn find_entries_2(xs: &[i32]) -> Option<(i32, i32)> {
  for (i, &x) in xs.iter().enumerate() {
    for (j, &y) in xs.iter().enumerate() {
      if i == j { continue; }
      if x + y == 2020 { return Some((x, y)) }
    }
  }
  None
}

fn find_entries_3(xs: &[i32]) -> Option<(i32, i32, i32)> {
  for (i, &x) in xs.iter().enumerate() {
    for (j, &y) in xs.iter().enumerate() {
      for (k, &z) in xs.iter().enumerate() {
        if i == j { continue; }
        if i == k { continue; }
        if j == k { continue; }
        if x + y + z == 2020 { return Some((x, y, z)) }
      }
    }
  }
  None
}
