#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IncRange {
    min: usize,
    max: usize,
}

#[derive(Debug, PartialEq, Eq)]
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
    let test_input = TestInput::from_file(&input_helpers::get_input_file_from_args(&mut std::env::args()));
    let err_rate = get_ticket_scanning_error_rate(&test_input.nearby_tickets, &test_input.rules);
    println!("Ticket scanning err rate: {}", err_rate);
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
}
