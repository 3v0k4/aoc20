use std::collections::HashMap;

fn main() {
  let file: String = std::fs::read_to_string("./input.txt")
    .expect("Something went wrong reading the file");
  println!("{}", solution_1(&file));
  println!("{}", solution_2(&file));
}

fn solution_1(xs: &String) -> usize {
  xs
    .split("\n\n")
    .map(|str| str.replace("\n", " "))
    .map(|str| str.trim().to_string())
    .filter(has_all_required_fields)
    .collect::<Vec<_>>()
    .len()
}

fn solution_2(xs: &String) -> usize {
  xs
    .split("\n\n")
    .map(|str| str.replace("\n", " "))
    .map(|str| str.trim().to_string())
    .filter(has_all_required_fields)
    .filter(has_valid_fields)
    .collect::<Vec<_>>()
    .len()
}

fn has_all_required_fields(document: &String) -> bool {
  document.contains("byr:") &&
    document.contains("iyr:") &&
    document.contains("eyr:") &&
    document.contains("hgt:") &&
    document.contains("hcl:") &&
    document.contains("ecl:") &&
    document.contains("pid:")
}

fn has_valid_fields(document: &String) -> bool {
  document
    .split(" ")
    .map(|field| {
      let key_val: Vec<_> = field.split(":").collect();
      (key_val[0], key_val[1])
    })
    .fold(HashMap::new(), |mut acc, (key, val)| { acc.insert(key, val); acc })
    .iter()
    .map(is_valid_field)
    .all(|x| x)
}

fn is_valid_field(key_val: (&&str, &&str)) -> bool {
  match (*key_val.0, *key_val.1) {
    ("byr", x) => validate_byr(x),
    ("iyr", x) => validate_iyr(x),
    ("eyr", x) => validate_eyr(x),
    ("hgt", x) => validate_hgt(x),
    ("hcl", x) => validate_hcl(x),
    ("ecl", x) => validate_ecl(x),
    ("pid", x) => validate_pid(x),
    _ => true
  }
}

fn validate_byr(x: &str) -> bool {
  let parsed: Result<u32, _> = x.parse();
  if let Ok(x) = parsed {
    x >= 1920 && x <= 2002
  } else {
    false
  }
}

fn validate_iyr(x: &str) -> bool {
  let parsed: Result<u32, _> = x.parse();
  if let Ok(x) = parsed {
    x >= 2010 && x <= 2020
  } else {
    false
  }
}

fn validate_eyr(x: &str) -> bool {
  let parsed: Result<u32, _> = x.parse();
  if let Ok(x) = parsed {
    x >= 2020 && x <= 2030
  } else {
    false
  }
}

fn validate_hgt(x: &str) -> bool {
  let rev: String = x.chars().rev().collect();
  let (rev_unit, rev_hgt) = rev.split_at(2);
  let hgt: String = rev_hgt.chars().rev().collect();
  let unit: String = rev_unit.chars().rev().collect();
  let parsed_hgt: Result<u32, _> = hgt.parse();
  match (unit.as_str(), parsed_hgt) {
    ("cm", Ok(p)) => p >= 150 && p <= 193,
    ("in", Ok(p)) => p >= 59 && p <= 76,
    _ => false
  }
}

fn validate_hcl(x: &str) -> bool {
  let (prefix, hcl) = x.split_at(1);
  let parsed: Result<String, _> = hcl.parse();
  match (prefix, parsed) {
    ("#", Ok(p)) => p.chars().all(valid_hcl),
    _ => false
  }
}

fn valid_hcl(x: char) -> bool {
  ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'].contains(&x)
}

fn validate_ecl(x: &str) -> bool {
  ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x)
}

fn validate_pid(x: &str) -> bool {
  x.len() == 9 &&
    x.chars().all(|y| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&y))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn validate_byr_valid() {
    assert!(validate_byr("2002"));
  }

  #[test]
  fn validate_byr_invalid() {
    assert!(!validate_byr("2003"));
  }

  #[test]
  fn validate_hgt_in_valid() {
    assert!(validate_hgt("60in"));
  }

  #[test]
  fn validate_hgt_cm_valid() {
    assert!(validate_hgt("190cm"));
  }

  #[test]
  fn validate_hgt_in_invalid() {
    assert!(!validate_hgt("190in"));
  }

  #[test]
  fn validate_hgt_cm_invalid() {
    assert!(!validate_hgt("190"));
  }

  #[test]
  fn validate_hcl_valid() {
    assert!(validate_hcl("#123abc"));
  }

  #[test]
  fn validate_hcl_invalid_1() {
    assert!(!validate_hcl("#123abz"));
  }

  #[test]
  fn validate_hcl_invalid_2() {
    assert!(!validate_hcl("123abc"));
  }

  #[test]
  fn validate_ecl_valid() {
    assert!(validate_ecl("brn"));
  }

  #[test]
  fn validate_ecl_invalid() {
    assert!(!validate_ecl("wat"));
  }

  #[test]
  fn validate_pid_valid() {
    assert!(validate_pid("000000001"));
  }

  #[test]
  fn validate_pid_invalid() {
    assert!(!validate_pid("0123456789"));
  }

  #[test]
  fn valid_all_valid() {
    let list = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string();

    assert_eq!(solution_2(&list), 4);
  }

  #[test]
  fn valid_none_valid() {
    let list = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007".to_string();

    assert_eq!(solution_2(&list), 0);
  }
}
