#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IncRange {
    min: usize,
    max: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TicketRule {
    field_name: String,
    valid_ranges: Vec<IncRange>,
}

type Ticket = Vec<usize>;

#[derive(Debug, PartialEq, Eq)]
struct TestInput {
    rules: Vec<TicketRule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn get_completely_invalid_ticket_value(ticket_values: &[usize], ticket_rules: &[TicketRule]) -> Option<usize> {
    for ticket_value in ticket_values {
        if ticket_rules.iter().all(|rule| rule.is_value_completely_invalid(*ticket_value)) {
            return Some(*ticket_value);
        }
    }

    None
}

fn get_ticket_scanning_error_rate(tickets: &[Ticket], ticket_rules: &[TicketRule]) -> usize {
    tickets.iter().map(|ticket| get_completely_invalid_ticket_value(&ticket, ticket_rules).unwrap_or(0)).sum()
}

fn discard_invalid_tickets(tickets: &[Ticket], ticket_rules: &[TicketRule]) -> Vec<Ticket> {
    tickets.iter().filter(|&ticket| get_completely_invalid_ticket_value(ticket, ticket_rules).is_none()).map(|e| e.clone()).collect()
}

fn classify_ticket_values(tickets: &[Ticket], ticket_rules: &[TicketRule]) -> Vec<String> {
    let rule_count = ticket_rules.len();

    let mut ordered_rule_names = vec![String::new();rule_count];
    ordered_rule_names.reserve(ticket_rules.len());

    let mut valid_rules_for_cols = Vec::new();

    for ticket_value_col in 0..rule_count { // there is one ticket_value_col for each rule
        let mut possible_matching_rules = ticket_rules.to_vec();
        for ticket_val in tickets.iter().map(|ticket| ticket[ticket_value_col]) {
            for i in (0..possible_matching_rules.len()).rev() {
                if possible_matching_rules[i].is_value_completely_invalid(ticket_val) {
                    possible_matching_rules.swap_remove(i);
                }
            }
        }

        valid_rules_for_cols.push((ticket_value_col, possible_matching_rules));
    }

    while !valid_rules_for_cols.is_empty() {
        let complete_rule_index = valid_rules_for_cols.iter().position(|(_, rules)| rules.len() == 1).unwrap();
        let (rule_index, mut complete_rule) = valid_rules_for_cols.swap_remove(complete_rule_index);
        let complete_rule = complete_rule.swap_remove(0);

        for (_, rules) in &mut valid_rules_for_cols {
            if let Some(index) = rules.iter().position(|ticket_rule| ticket_rule.field_name == complete_rule.field_name) {
                rules.swap_remove(index);
            }
        }

        ordered_rule_names[rule_index] = complete_rule.field_name;
    }

    ordered_rule_names
}

impl IncRange {
    fn new(min: usize, max: usize) -> Self {
        IncRange {min, max}
    }

    fn is_in_range(&self, v: usize) -> bool {
        self.min <= v && v <= self.max 
    }
}

impl TicketRule {
    fn new(field_name: &str, valid_ranges: &[IncRange]) -> Self {
        TicketRule {
            field_name: String::from(field_name),
            valid_ranges: valid_ranges.to_vec(),
        }
    }

    fn is_value_completely_invalid(&self, v: usize) -> bool {
        self.valid_ranges.iter().all(|r| !r.is_in_range(v))
    }
}

impl TestInput {
    fn parse_rule(line: &str) -> TicketRule {
        let mut valid_ranges = Vec::new();

        let (field_name, range_strs) = {
            let mut name_ranges_split = line.split(": ");
            let field_name = name_ranges_split.next().unwrap();
            let range_strs = name_ranges_split.next().unwrap();
            (field_name, range_strs)
        };

        for range_str in range_strs.split(" or ") {
            let mut range_split = range_str.split("-");
            let min = range_split.next().unwrap().parse().unwrap();
            let max = range_split.next().unwrap().parse().unwrap();
            valid_ranges.push(IncRange::new(min, max));
        }

        TicketRule::new(field_name, &valid_ranges)
    }

    fn parse_ticket(line: &str) -> Ticket {
        line.split(',').map(|ticket_value| ticket_value.parse::<usize>().unwrap()).collect()
    }

    fn from_file(file_name: &str) -> Self {
        let mut rules = Vec::new();
        let mut my_ticket = Ticket::new();
        let mut nearby_tickets = Vec::new();

        #[derive(PartialEq, Eq)]
        enum ParseState {
            ParsingRules,
            ParsingMyTicket,
            ParsingNearbyTickets,
        }

        let mut state = ParseState::ParsingRules;

        for line in input_helpers::read_lines(file_name) {
            if line == "" {
                continue;
            }
            else if line == "your ticket:" {
                assert!(state == ParseState::ParsingRules);
                state = ParseState::ParsingMyTicket;
                continue;
            }
            else if line == "nearby tickets:" {
                assert!(state == ParseState::ParsingMyTicket);
                state = ParseState::ParsingNearbyTickets;
                continue;
            }

            match state {
                ParseState::ParsingRules => rules.push(TestInput::parse_rule(&line)),
                ParseState::ParsingMyTicket => my_ticket = TestInput::parse_ticket(&line),
                ParseState::ParsingNearbyTickets => nearby_tickets.push(TestInput::parse_ticket(&line)),
            }
        }

        TestInput {
            rules,
            my_ticket,
            nearby_tickets,
        }
    } 
}

fn main() {
    let test_file_name = input_helpers::get_input_file_from_args(&mut std::env::args());
    let test_input = TestInput::from_file(&test_file_name);
    let err_rate = get_ticket_scanning_error_rate(&test_input.nearby_tickets, &test_input.rules);
    println!("Ticket scanning err rate: {}", err_rate);

    let filtered_tickets = discard_invalid_tickets(&test_input.nearby_tickets, &test_input.rules);
    let ordered_rules = classify_ticket_values(&filtered_tickets, &test_input.rules);
    println!("Each column's rule: {:?}", ordered_rules);

    let departure_rules = ordered_rules.iter().enumerate().filter(|(_, rule_name)| rule_name.starts_with("departure"));
    let mut departure_values = std::collections::HashMap::new();
    for (rule_index, rule_name) in departure_rules {
        departure_values.insert(rule_name, test_input.my_ticket[rule_index]);
    }
    println!("my ticket's departure rows: {:?}", departure_values);

    let departure_values_product: usize = departure_values.iter().map(|(_, v)| v).product();
    println!("Product of departure values: {}", departure_values_product);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_simple_test_input() -> TestInput {
        // class: 1-3 or 5-7
        // row: 6-11 or 33-44
        // seat: 13-40 or 45-50
        // 
        // your ticket:
        // 7,1,14
        // 
        // nearby tickets:
        // 7,3,47 (valid)
        // 40,**4**,50 (invalid)
        // **55**,2,20 (invalid)
        // 38,6,**12** (invalid)
        let rules = vec![
            TicketRule::new("class", &[IncRange::new(1, 3), IncRange::new(5, 7)]),
            TicketRule::new("row", &[IncRange::new(6, 11), IncRange::new(33, 44)]),
            TicketRule::new("seat", &[IncRange::new(13, 40), IncRange::new(45, 50)]),
        ];

        let my_ticket = vec![7, 1, 14];

        let nearby_tickets = vec![
            vec![7, 3, 47], // valid
            vec![40, 4, 50], // invalid (4)
            vec![55, 2, 20], // invalid (55)
            vec![38, 6, 12], // invalid (12)
            ];

        TestInput {
            rules,
            my_ticket,
            nearby_tickets,
        }
    }

    #[test]
    fn completely_invalid_ticket_test() {
        let simple_test_input = get_simple_test_input();
        let ticket_rules = simple_test_input.rules;
        let tickets = simple_test_input.nearby_tickets;

        assert_eq!(get_completely_invalid_ticket_value(&tickets[0], &ticket_rules), None);
        assert_eq!(get_completely_invalid_ticket_value(&tickets[1], &ticket_rules), Some(4));
        assert_eq!(get_completely_invalid_ticket_value(&tickets[2], &ticket_rules), Some(55));
        assert_eq!(get_completely_invalid_ticket_value(&tickets[3], &ticket_rules), Some(12));
        assert_eq!(get_ticket_scanning_error_rate(&tickets, &ticket_rules), 71);
    }

    #[test]
    fn test_input_parsing( ) {
        let test_input = TestInput::from_file("src/simple_input.txt");
        let expected_test_input = get_simple_test_input();
        assert_eq!(test_input, expected_test_input);
    }

    #[test]
    fn test_col_classification() {
        let simple_test_input = TestInput::from_file("src/simple2_input.txt");
        let ordered_rules = classify_ticket_values(&simple_test_input.nearby_tickets, &simple_test_input.rules);
        assert_eq!(ordered_rules, ["row", "class", "seat"]);
    }
}