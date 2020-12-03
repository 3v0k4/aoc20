use std::collections::HashSet;
use std::convert::TryInto;

struct Map {
  trees: HashSet<(u32, u32)>,
  height: u32,
  width: u32
}

struct Slope {
  right: u32,
  down: u32
}

fn main() {
    let map = map();

    let slopes = [
      Slope { right: 3, down: 1 },
      Slope { right: 1, down: 1 },
      Slope { right: 5, down: 1 },
      Slope { right: 7, down: 1 },
      Slope { right: 1, down: 2 }
    ];

    let solution1 = traverse(&map, &slopes[0]);
    println!("{}", solution1);

    let solution2 = slopes
      .iter()
      .map(|slope| traverse(&map, slope))
      .product::<u32>();
    println!("{}", solution2);
}

fn map() -> Map {
  let mut height: u32 = 0;
  let mut width: u32 = 0;
  let mut trees: HashSet<(u32, u32)> = HashSet::new();

  std::fs::read_to_string("./input.txt")
    .expect("Something went wrong reading the file")
    .lines()
    .enumerate()
    .for_each(|(i, row)| {
      row.chars().enumerate().for_each(|(j, col)| {
        let jj: u32 = j.try_into().unwrap();
        let ii: u32 = i.try_into().unwrap();
        if col == '#' { trees.insert((jj + 1, ii + 1)); }
        if jj + 1 > width { width = jj + 1 }
        if ii + 1 > height { height = ii + 1 }
      })
    });

  Map { trees, height, width }
}

fn traverse(map: &Map, slope: &Slope) -> u32 {
    let mut coord = (1, 1);
    let mut count = 0;
    while coord.1 <= map.height {
      let row = ((coord.0 + slope.right - 1) % map.width) + 1;
      let col = coord.1 + slope.down;
      coord = (row, col);
      if map.trees.contains(&coord) { count += 1 }
    }
    count
}
