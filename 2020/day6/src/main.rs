type MemberAnswers = String;
type GroupAnswers = Vec<MemberAnswers>;

fn get_group_answers_from_input(file_name: &str) -> Vec<GroupAnswers> {
    let mut group_answers_list = Vec::new();
    let mut next_group_answers = GroupAnswers::new();

    for line in input_helpers::read_lines(file_name) {
        match line.as_str() {
            "" => group_answers_list.push(std::mem::replace(&mut next_group_answers, GroupAnswers::new())),
            _ => next_group_answers.push(line),
        }
    }

    if !next_group_answers.is_empty() {
        group_answers_list.push(std::mem::replace(&mut next_group_answers, GroupAnswers::new()));
    }

    group_answers_list
}

fn main() {
    for group_answers in get_group_answers_from_input("src/simple_input.txt") {
        println!("member count: {}", group_answers.len());
    }
}
