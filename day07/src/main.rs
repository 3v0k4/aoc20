use petgraph::graph::{Graph};
use petgraph::algo::{has_path_connecting, all_simple_paths};

fn main() {
    let file: String = std::fs::read_to_string("./input.txt").unwrap();
    let graph = graph_it_up(&file);
    println!("{}", solution_1(&graph));
    println!("{}", solution_2(&graph));
}

fn graph_it_up(rules: &str) -> Graph<String, u32> {
    let mut graph = Graph::<String, u32>::new();
    parse_lines(rules)
        .iter()
        .for_each(|(from, tos)| {
            let from_node = graph
                .node_indices()
                .find(|i| graph[*i] == from.0)
                .unwrap_or_else(|| graph.add_node(from.0.clone()));
            tos
                .iter()
                .for_each(|to| {
                    let to_node = graph
                        .node_indices()
                        .find(|i| graph[*i] == to.0)
                        .unwrap_or_else(|| graph.add_node(to.0.clone()));
                    graph.add_edge(from_node, to_node, to.1);
                })
        });
    graph
}

fn solution_1(graph: &Graph<String, u32>) -> usize {
    let to = graph.node_indices().find(|i| graph[*i] == "shiny gold").unwrap();
    graph
        .node_indices()
        .filter(|from| has_path_connecting(&graph, *from, to, None))
        .count() - 1
}

fn solution_2(graph: &Graph<String, u32>) -> u32 {
    let from = graph.node_indices().find(|i| graph[*i] == "shiny gold").unwrap();
    graph
        .node_indices()
        .flat_map(|to| all_simple_paths(&graph, from, to, 0, None).collect::<Vec<_>>())
        .map(|path: Vec<_>| {
            path
                .windows(2)
                .map(|window| graph.find_edge(window[0], window[1]).unwrap())
                .map(|edge| graph[edge])
                .product::<u32>()
        })
        .sum::<u32>()
}

fn parse_lines(rules: &str) -> Vec<((String, u32), Vec<(String, u32)>)> {
    rules
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(rule: &str) -> ((String, u32), Vec<(String, u32)>) {
    let replaced = rule.replace(" bags", "").replace(" bag", "").replace(".", "");
    let split: Vec<&str> = replaced
        .split(" contain ")
        .collect();
    let container = (split[0].to_string(), 0);
    let containees: Vec<(String, u32)> = split[1]
        .split(", ")
        .map(|containee_string| {
            if containee_string == "no other" { return ("".to_string(), 0) }
            let parts: Vec<&str> = containee_string.split(" ").collect();
            let containee = parts[1..].join(" ");
            let amount: u32 = parts[0].parse().unwrap();
            (containee, amount)
        })
        .collect();
    (container, containees)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            (("light red".to_string(), 0), vec![("bright white".to_string(), 1), ("muted yellow".to_string(), 2)])
        );
        assert_eq!(
            parse_line("bright white bags contain 1 shiny gold bag."),
            (("bright white".to_string(), 0), vec![("shiny gold".to_string(), 1)])
        );
        assert_eq!(
            parse_line("faded blue bags contain no other bags."),
            (("faded blue".to_string(), 0), vec![])
        );
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags."),
            vec![
                (("vibrant plum".to_string(), 0), vec![("faded blue".to_string(), 5), ("dotted black".to_string(), 6)]),
                (("faded blue".to_string(), 0), vec![])
            ]
        );
    }

    #[test]
    fn test_solution_1() {
        let graph = graph_it_up("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");
        assert_eq!(solution_1(&graph), 4);
    }

    #[test]
    fn test_solution_2() {
        let graph = graph_it_up("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");
        assert_eq!(solution_2(&graph), 32);
    }
}
