use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Instruction {
  Jmp(i32, Changed),
  Acc(i32),
  Nop(i32, Changed),
}

#[derive(Copy, Clone)]
enum Changed {
    No,
    Yes,
    Reverted,
}

fn main() {
    let file: String = std::fs::read_to_string("./input.txt").unwrap();
    let program = parse_file(&file);
    println!("{:?}", execute(&program));
    println!("{:?}", execute_with_patch(&program));
}

fn parse_file(file: &str) -> Vec<Instruction> {
    file
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Instruction {
    match line.split(" ").collect::<Vec<_>>()[..] {
        ["nop", x] => Instruction::Nop(x.parse::<i32>().unwrap(), Changed::No),
        ["acc", x] => Instruction::Acc(x.parse::<i32>().unwrap()),
        ["jmp", x] => Instruction::Jmp(x.parse::<i32>().unwrap(), Changed::No),
        _ => panic!("Unexpected instruction {}", line),
    }
}

fn execute(program: &Vec<Instruction>) -> (i32, bool) {
    let mut ip: i32 = 0;
    let mut acc: i32 = 0;
    let mut executed: HashSet<i32> = HashSet::new();
    loop {
        if executed.contains(&ip) { return (acc, false) }
        if ip as usize == program.len() { return (acc, true) }
        executed.insert(ip);
        match program[ip as usize] {
            Instruction::Nop(_, _) => ip += 1,
            Instruction::Acc(x) => { acc += x; ip += 1 },
            Instruction::Jmp(x, _) => ip += x,
        }
    }
}

fn execute_with_patch(program: &Vec<Instruction>) -> i32 {
    let mut patched = program.to_vec();
    loop {
        match execute(&patched) {
            (acc, true) => return acc,
            (_, false) => patched = patch(&patched),
        }
    }
}

fn patch(program: &Vec<Instruction>) -> Vec<Instruction> {
    let mut flipped = false;
    program
        .iter()
        .map(|instruction| {
            if flipped { return *instruction }
            match instruction {
                Instruction::Nop(x, Changed::No) => {
                    flipped = true;
                    Instruction::Jmp(*x, Changed::Yes)
                },
                Instruction::Jmp(x, Changed::No) => {
                    flipped = true;
                    Instruction::Nop(*x, Changed::Yes)
                },
                Instruction::Nop(x, Changed::Yes) => {
                    Instruction::Jmp(*x, Changed::Reverted)
                },
                Instruction::Jmp(x, Changed::Yes) => {
                    Instruction::Nop(*x, Changed::Reverted)
                },
                instruction => *instruction,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_executes() {
        let program = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(execute(&parse(program)), (5, false));

        let program = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";
        assert_eq!(execute(&parse(program)), (8, true));
    }

   #[test]
   fn it_executes_with_patch() {
       let program = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";
       assert_eq!(execute_with_patch(&parse(program)), 8);
   }
}
