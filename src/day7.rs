use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;
use petgraph::graph::DiGraph;
use petgraph::matrix_graph::NodeIndex;
use std::collections::HashMap;

struct BagRule {
    colour: String,
    content: HashMap<String, u32>,
}

pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day7-input")?.parse()?;

    let rules = parse_rules(&input);

    let (graph, nodes) = rules_to_graph(&rules);
    let result_1 = calc_ways_to_colour("shiny gold", &graph, &nodes);
    let result_2 = calc_bags_inside("shiny gold", &graph, &nodes);

    Ok((format!("{}", result_1), format!("{}", result_2)))
}

fn from_bag_str(
    input: (&str, &str, &str, &str, &str, &str, &str),
) -> Result<String, std::num::ParseIntError> {
    Ok(input.0.to_owned() + " " + input.2)
}

fn from_end(input: &str) -> Result<bool, std::num::ParseIntError> {
    Ok(input == ".")
}

fn from_empty_content_bag_str(_: &str) -> Result<(u32, String), std::num::ParseIntError> {
    Ok((0u32, "other".to_owned()))
}

fn from_content_bag_str(
    input: (&str, &str, &str, &str, &str, &str),
) -> Result<(u32, String), std::num::ParseIntError> {
    if input.0 == "no" {
        return Ok((0u32, input.2.to_owned() + " " + input.4));
    }
    Ok((
        u32::from_str_radix(input.0, 10)?,
        input.2.to_owned() + " " + input.4,
    ))
}

fn initial_bag(input: &str) -> IResult<&str, String> {
    map_res(
        tuple((
            alpha1,
            tag(" "),
            alpha1,
            tag(" "),
            alpha1,
            tag(" "),
            tag("contain "),
        )),
        from_bag_str,
    )(input)
}

fn content_bag(input: &str) -> IResult<&str, (u32, String)> {
    let res = map_res(tag("no other bags"), from_empty_content_bag_str)(input);
    if res.is_ok() {
        res
    } else {
        map_res(
            tuple((
                digit1,
                tag(" "),
                alpha1,
                tag(" "),
                alpha1,
                alt((tag(" bags"), tag(" bag"))),
            )),
            from_content_bag_str,
        )(input)
    }
}

fn end_or_continue(input: &str) -> IResult<&str, bool> {
    map_res(alt((tag(", "), tag("."))), from_end)(input)
}

fn parse_rule(line: &str) -> BagRule {
    let (mut line, colour) = initial_bag(line).unwrap();

    let mut content = HashMap::new();
    let mut do_loop = true;
    while do_loop {
        let (int_line, content_bag) = content_bag(line).unwrap();
        let (int_line, end) = end_or_continue(int_line).unwrap();
        content.insert(content_bag.1, content_bag.0);
        do_loop = !end;
        line = int_line;
    }

    BagRule { colour, content }
}

fn parse_rules(input: &str) -> HashMap<String, BagRule> {
    let lines = input.trim().split('\n');
    let mut rules = HashMap::new();

    for line in lines {
        let rule = parse_rule(line);

        rules.insert(rule.colour.clone(), rule);
    }

    rules
}

fn rules_to_graph(
    rules: &HashMap<String, BagRule>,
) -> (DiGraph<String, u32>, HashMap<String, NodeIndex<u32>>) {
    let mut ret = DiGraph::<String, u32>::new();
    let mut nodes = HashMap::<String, NodeIndex<u32>>::new();

    for rule in rules.values() {
        if !nodes.contains_key(&rule.colour) {
            nodes.insert(rule.colour.clone(), ret.add_node(rule.colour.clone()));
        }
        for target in rule.content.keys() {
            if !nodes.contains_key(target) {
                nodes.insert(target.clone(), ret.add_node(target.clone()));
            }

            let from = nodes.get(&rule.colour).unwrap();
            let to = nodes.get(target).unwrap();
            ret.add_edge(*from, *to, *rule.content.get(target).unwrap());
        }
    }

    (ret, nodes)
}

fn calc_ways_to_colour(
    target_colour: &str,
    graph: &DiGraph<String, u32>,
    nodes: &HashMap<String, NodeIndex<u32>>,
) -> usize {
    let mut num_ways = 0;

    let target = nodes.get(target_colour).unwrap();
    for s in nodes.keys() {
        if s == target_colour {
            continue;
        }
        let source = nodes.get(s).unwrap();

        num_ways +=
            if petgraph::algo::all_simple_paths::<Vec<_>, _>(graph, *source, *target, 0, None)
                .count()
                > 0
            {
                1
            } else {
                0
            };
    }

    num_ways
}

fn calc_bags_inside(
    source_colour: &str,
    graph: &DiGraph<String, u32>,
    nodes: &HashMap<String, NodeIndex<u32>>,
) -> u32 {
    let mut num_bags = 0;

    let source = nodes.get(source_colour).unwrap();
    for t in nodes.keys() {
        if t == source_colour {
            continue;
        }
        let target = nodes.get(t).unwrap();

        let ways = petgraph::algo::all_simple_paths::<Vec<_>, _>(graph, *source, *target, 0, None)
            .collect::<Vec<_>>();
        for way in ways {
            num_bags += cost_of_path(&way, graph);
        }
    }

    num_bags
}

fn cost_of_path(path: &[NodeIndex<u32>], graph: &DiGraph<String, u32>) -> u32 {
    let mut cost = 1;
    for (i, node) in path.iter().enumerate() {
        if i == 0 {
            continue;
        }
        for edge in graph.edges_connecting(path[i - 1], *node) {
            cost *= edge.weight();
        }
    }
    cost
}

#[test]
fn test_parse() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let rules = parse_rules(input);

    assert_eq!(9, rules.len());

    let (graph, nodes) = rules_to_graph(&rules);
    let result_1 = calc_ways_to_colour("shiny gold", &graph, &nodes);

    assert_eq!(4, result_1);
}

#[test]
fn test_part2() {
    let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    let rules = parse_rules(input);

    assert_eq!(7, rules.len());

    let (graph, nodes) = rules_to_graph(&rules);
    let result_2 = calc_bags_inside("shiny gold", &graph, &nodes);

    assert_eq!(126, result_2);
}
