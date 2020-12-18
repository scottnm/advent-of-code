use std::iter::FromIterator;

#[derive(Debug, Clone, Copy)]
struct IncRange {
    min: usize,
    max: usize,
}

impl IncRange {
    fn new(min: usize, max: usize) -> Self {
        IncRange {min, max}
    }

    fn is_in_range(&self, v: usize) -> bool {
        self.min <= v && v <= self.max 
    }
}

struct TicketRule {
    field_name: String,
    valid_ranges: Vec<IncRange>,
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

fn get_completely_invalid_ticket_value(ticket_values: &[usize], ticket_rules: &[TicketRule]) -> Option<usize> {
    for ticket_value in ticket_values {
        if ticket_rules.iter().all(|rule| rule.is_value_completely_invalid(*ticket_value)) {
            return Some(*ticket_value);
        }
    }

    None
}

type Ticket = Vec<usize>;

fn get_ticket_scanning_error_rate(tickets: &[Ticket], ticket_rules: &[TicketRule]) -> usize {
    tickets.iter().map(|ticket| get_completely_invalid_ticket_value(&ticket, ticket_rules).unwrap_or(0)).sum()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completely_invalid_ticket_test() {
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
        let ticket_rules = [
            TicketRule::new("class", &[IncRange::new(1, 3), IncRange::new(5, 7)]),
            TicketRule::new("row", &[IncRange::new(6, 11), IncRange::new(33, 44)]),
            TicketRule::new("seat", &[IncRange::new(13, 40), IncRange::new(45, 50)]),
        ];

        let tickets = [
            vec![7, 3, 47], // valid
            vec![40, 4, 50], // invalid (4)
            vec![55, 2, 20], // invalid (55)
            vec![38, 6, 12], // invalid (12)
            ];

        assert_eq!(get_completely_invalid_ticket_value(&tickets[0], &ticket_rules), None);
        assert_eq!(get_completely_invalid_ticket_value(&tickets[1], &ticket_rules), Some(4));
        assert_eq!(get_completely_invalid_ticket_value(&tickets[2], &ticket_rules), Some(55));
        assert_eq!(get_completely_invalid_ticket_value(&tickets[3], &ticket_rules), Some(12));
        assert_eq!(get_ticket_scanning_error_rate(&tickets, &ticket_rules), 71);
    }
}
