#[macro_use] extern crate lazy_static;
extern crate regex;

type RuleId = String;
struct InvertedRulesMap{
    map: std::collections::HashMap<RuleId, Vec<RuleId>>,
}

impl InvertedRulesMap {
    fn count_to_dest(&self, dest_rule: &str) -> usize {
        // Using breadth-first semantics, start at the destination and work backwards keeping
        // track of each rule which can lead back to that destination
        let mut current_rule = String::from(dest_rule);
        let mut route_set = std::collections::HashSet::new();
        let mut paths_to_check = Vec::new();

        let empty_vec: Vec<RuleId> = vec![];
        loop {
            let pathes_to_rule = self.map.get(&current_rule).unwrap_or(&empty_vec);
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

        route_set.len()
    }
}

fn get_rules_from_input(file_name: &str) -> InvertedRulesMap {
    let mut map = std::collections::HashMap::<RuleId, Vec<RuleId>>::new();

    fn parse_bag_from_containing_rule(containing_rule_str: &str) -> RuleId {
        lazy_static! {
            // EXAMPLES:
            // light red bags contain 1 bright white bag, 2 muted yellow bags.
            // bright white bags contain 1 shiny gold bag.
            // faded blue bags contain no other bags.

            static ref BAG_REGEX: regex::Regex =
                regex::Regex::new(r"\d (.+) bag").unwrap();
        }

        let captures = BAG_REGEX.captures(containing_rule_str).unwrap();
        String::from(&captures[1])
    }

    fn parse_rule_from_line(line: &str) -> (RuleId, Vec<RuleId>) {
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
            _ => containing_rules_str.split(',').map(|rule| parse_bag_from_containing_rule(rule)).collect(),
        };

        (String::from(dest_rule), containing_rules)
    }

    for line in input_helpers::read_lines(file_name) {
        let (dest_rule, containing_rules) = parse_rule_from_line(&line);
        println!("{} - {:?}", dest_rule, containing_rules);
        map.insert(dest_rule, containing_rules);
    }

    InvertedRulesMap { map }
}

fn main() {
    let inverted_rules_map = get_rules_from_input("src/simple_input.txt");
    let dest_rule = "shiny gold";
    let cnt = inverted_rules_map.count_to_dest(dest_rule);
    println!("# routes to {}: {}", dest_rule, cnt);
}