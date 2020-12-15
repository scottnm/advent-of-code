#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{HashMap, HashSet};

type RuleId = String;

struct RulesMap {
    top_down: HashMap<RuleId, Vec<(RuleId, usize)>>,
    bottom_up: HashMap<RuleId, HashSet<RuleId>>,
}

impl RulesMap {
    fn count_to_dest(&self, dest_rule: &str) -> usize {
        // Using breadth-first semantics, start at the destination and work backwards keeping
        // track of each rule which can lead back to that destination
        let mut current_rule = String::from(dest_rule);
        let mut route_set = HashSet::new();
        let mut paths_to_check = Vec::new();

        let empty_hashset = HashSet::<RuleId>::new();
        loop {
            let pathes_to_rule = self.bottom_up.get(&current_rule).unwrap_or(&empty_hashset);
            for path in pathes_to_rule {
                let is_new_route = route_set.insert(path.clone());
                if is_new_route {
                    paths_to_check.push(path.clone());
                }
            }

            if paths_to_check.is_empty() {
                break;
            }
            current_rule = paths_to_check.pop().unwrap().clone();
        }

        println!("{:?}", route_set);
        route_set.len()
    }

    fn count_total_contained(&self, src_bag: &str) -> usize {
        let mut current_bag = String::from(src_bag);
        let mut all_bags = Vec::new();
        let mut bags_to_check = Vec::new();

        let empty_vec = Vec::new();
        loop {
            let contained_bag_rules = self.top_down.get(&current_bag).unwrap_or(&empty_vec);
            for (contained_bag, count) in contained_bag_rules {
                for _ in 0..*count {
                    bags_to_check.push(contained_bag.clone());
                    all_bags.push(contained_bag.clone());
                }
            }

            if bags_to_check.is_empty() {
                break;
            }
            current_bag = bags_to_check.pop().unwrap().clone();
        }

        println!("{:?}", all_bags);
        all_bags.len()
    }
}

fn get_rules_from_input(file_name: &str) -> RulesMap {
    fn parse_bag_from_containing_rule(containing_rule_str: &str) -> (RuleId, usize) {
        lazy_static! {
            // EXAMPLES:
            // light red bags contain 1 bright white bag, 2 muted yellow bags.
            // bright white bags contain 1 shiny gold bag.
            // faded blue bags contain no other bags.

            static ref BAG_REGEX: regex::Regex =
                regex::Regex::new(r"(\d+) (.+) bag").unwrap();
        }

        let captures = BAG_REGEX.captures(containing_rule_str).unwrap();
        (String::from(&captures[2]), captures[1].parse().unwrap())
    }

    fn parse_rule_from_line(line: &str) -> (RuleId, Vec<(RuleId, usize)>) {
        lazy_static! {
            // EXAMPLES:
            // light red bags contain 1 bright white bag, 2 muted yellow bags.
            // bright white bags contain 1 shiny gold bag.
            // faded blue bags contain no other bags.

            static ref LINE_REGEX: regex::Regex =
                regex::Regex::new(r"(.+) bags contain (.*).").unwrap();
        }

        let captures = LINE_REGEX.captures(line).unwrap();
        let dest_rule = &captures[1];
        let containing_rules_str = &captures[2];

        let containing_rules = match containing_rules_str {
            "no other bags" => vec![],
            _ => containing_rules_str
                .split(',')
                .map(|rule| parse_bag_from_containing_rule(rule))
                .collect(),
        };

        (String::from(dest_rule), containing_rules)
    }

    let mut top_down = HashMap::<RuleId, Vec<(RuleId, usize)>>::new();
    let mut bottom_up = HashMap::<RuleId, HashSet<RuleId>>::new();

    for line in input_helpers::read_lines(file_name) {
        let (containing_bag, dest_rules) = parse_rule_from_line(&line);
        println!("{} - {:?}", containing_bag, dest_rules);

        top_down.insert(containing_bag.clone(), dest_rules.clone());
        for (dest, _) in &dest_rules {
            if !bottom_up.contains_key(dest) {
                bottom_up.insert(dest.clone(), HashSet::new());
            }

            bottom_up
                .get_mut(dest)
                .unwrap()
                .insert(containing_bag.clone());
        }
    }

    RulesMap {
        top_down,
        bottom_up,
    }
}

fn main() {
    let bag_type = "shiny gold";
    let rules_map = get_rules_from_input("src/input.txt");

    let cnt_to_dest = rules_map.count_to_dest(bag_type);
    println!("# routes to {}: {}", bag_type, cnt_to_dest);

    let cnt_total_contained = rules_map.count_total_contained(bag_type);
    println!("# bags in {}: {}", bag_type, cnt_total_contained);
}
