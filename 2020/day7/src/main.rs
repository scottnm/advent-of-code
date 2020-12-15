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

        loop {
            let pathes_to_rule = &self.map[&current_rule];
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

fn get_rules_from_input(_file_name: &str) -> InvertedRulesMap {
    let mut map = std::collections::HashMap::<RuleId, Vec<RuleId>>::new();

    let light_red = String::from("light red");
    let bright_white = String::from("bright white");
    let muted_yellow = String::from("muted yellow");
    let dark_orange = String::from("dark orange");
    let shiny_gold = String::from("shiny gold");
    let dark_olive = String::from("dark olive");
    let vibrant_plum = String::from("vibrant plum");
    let faded_blue = String::from("faded blue");
    let dotted_black = String::from("dotted black");

    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    map.insert(light_red.clone(), vec![bright_white.clone(), muted_yellow.clone()]);
    // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    map.insert(dark_orange.clone(), vec![bright_white.clone(), muted_yellow.clone()]);
    // bright white bags contain 1 shiny gold bag.
    map.insert(bright_white.clone(), vec![shiny_gold.clone()]);
    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    map.insert(muted_yellow.clone(), vec![shiny_gold.clone(), faded_blue.clone()]);
    // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    map.insert(shiny_gold.clone(), vec![dark_olive.clone(), vibrant_plum.clone()]);
    // dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    map.insert(dark_olive.clone(), vec![faded_blue.clone(), dotted_black.clone()]);
    // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    map.insert(vibrant_plum.clone(), vec![faded_blue.clone(), dotted_black.clone()]);
    // faded blue bags contain no other bags.
    map.insert(faded_blue.clone(), vec![]);
    // dotted black bags contain no other bags.
    map.insert(dotted_black.clone(), vec![]);

    InvertedRulesMap { map }
}

fn main() {
    let inverted_rules_map = get_rules_from_input("src/input.txt");
    let dest_rule = "shiny gold";
    let cnt = inverted_rules_map.count_to_dest(dest_rule);
    println!("# routes to {}: {}", dest_rule, cnt);
}