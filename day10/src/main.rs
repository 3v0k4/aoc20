fn main() {
    let file: String = std::fs::read_to_string("./input.txt").unwrap();
    let mut adapters: Vec<u32> = file.lines().map(|x| x.parse().unwrap()).collect();
    adapters.sort_unstable();
    println!("{}", solution_1(&adapters));
    println!("{}", solution_2(&adapters));
}

fn solution_1(adapters: &Vec<u32>) -> u32 {
    let mut with_charging_outlet = adapters.clone();
    with_charging_outlet.insert(0, 0);
    let (ones, threes) = adapters
        .iter()
        .zip(with_charging_outlet)
        .fold((0, 1), |acc, xs| match xs.0 - xs.1 {
            3 => (acc.0, acc.1 + 1),
            1 => (acc.0 + 1, acc.1),
            _ => acc,
        });
    ones * threes
}

// All the credit for this algorithm goes to @basile_henry.
//
// What follows is how I understand the intuition behind it.
//
// Each adapter can connect only to a previous_adapter that
// is 1, 2, or 3 jolts away. This explains the `filter()`.
//
// The state in the `fold()` represents how many `ways` there
// are to get to that adapter.
//
// For example, given [3, 6] as `adapters`, the chain would
// be [0, 3, 6, 9] (0 is the `charging_outlet` and 9 is the
// `device`). There is only one distinct way to arrange the
// adapters to connect the charging_outlet to the device,
// which is using both 3 and 6.  Removing any of the two would
// create a gap bigger than 3 jolts, which is invalid.
//
// Given [0, 3, 6, 9], the accumulator in the fold would evolve
// as follows:
// [(1, 0)] -> There is only one way to get to 0
// [(1, 3), (1, 0)] -> There is only one way to get to 3
// [(1, 6), (1, 3), (1, 0)] -> There is only one way to get to 6
// [(1, 9), (1, 6), (1, 3), (1, 0)] -> There is only one way to get to 9
// Solution -> 1
//
// Let's try a situation in which there's more than one way, [0, 1, 2, 5]:
// [(1, 0)]
// [(1, 1), (1, 0)]
// [(2, 2), (1, 1), (1, 0)] -> Two ways of getting to 2 (i.e., keep or skip 1)
// [(2, 5), (2, 2), (1, 1), (1, 0)]
// Solution -> 2
//
// Notice there's one way to get to 0 and 1. Since 2 can see both
// (2 - 1 <= 3 and 2 - 0 <= 3), then there are 1 + 1 ways to get to 2. This
// explains the `sum()`.
//
// The `truncate()` is an optimization (the algorithm works without it) to
// prevent the accumulator from growing too long. Since there cannot be
// duplicate adapters (i.e., with the same jolt) and they are sorted, only
// the last three adapters are relevant to calculate the ways to get to the
// current one.

fn solution_2(adapters: &Vec<u32>) -> usize {
    let mut adapters = adapters.clone();
    let device = adapters.last().unwrap() + 3;
    adapters.push(device);
    let charging_outlet = 0;

    adapters
        .iter()
        .fold(vec![(1, charging_outlet)], |mut acc, &adapter| {
            let count = acc
                .iter()
                .filter(|(_, previous_adapter)| adapter - previous_adapter <= 3)
                .map(|(ways, _)| ways)
                .sum();

            acc.insert(0, (count, adapter));
            acc.truncate(3);
            acc
        })[0].0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solution_1() {
        let mut xs = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4].to_vec();
        xs.sort_unstable();
        assert_eq!(solution_1(&xs), 35);
    }

    #[test]
    fn it_solution2() {
        let mut xs = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4].to_vec();
        xs.sort_unstable();
        assert_eq!(solution_2(&xs), 8);
    }
}
