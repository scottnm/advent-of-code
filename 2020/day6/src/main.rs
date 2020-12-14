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

fn _count_questions_with_yes_answer(group_answers: &GroupAnswers) -> usize {
    let mut yes_answers = [false; 26];
    for member_answers in group_answers {
        for answer in member_answers.chars() {
            assert!(answer.is_ascii());
            let answer_index = (answer as usize) - ('a' as usize);
            yes_answers[answer_index] = true;
        }
    }

    yes_answers.iter().filter(|a| **a).count()
}

fn count_questions_with_only_yes_answers(group_answers: &GroupAnswers) -> usize {
    let mut yes_answers = [0usize; 26];
    for member_answers in group_answers {
        for answer in member_answers.chars() {
            assert!(answer.is_ascii());
            let answer_index = (answer as usize) - ('a' as usize);
            yes_answers[answer_index] += 1;
        }
    }

    let member_count = group_answers.len();
    yes_answers.iter().filter(|yes_answer_count| **yes_answer_count == member_count).count()
}

fn main() {
    let group_answers_list = get_group_answers_from_input("src/input.txt");
    for group_answers in &group_answers_list {
        println!("member count: {}", group_answers.len());
    }

    let any_yes_answer_counts: Vec<usize> = group_answers_list.iter().map(|g| count_questions_with_only_yes_answers(g)).collect();
    let total_any_yes_answer_count = any_yes_answer_counts.iter().fold(0, |a, b| a + b);
    println!("Answer: {}", total_any_yes_answer_count);
}
