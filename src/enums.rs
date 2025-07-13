enum ExaminationResult {
    Passed(String),
    NotPassed(String),
}

fn determine_student_result(scores: (i32, i32, i32)) -> ExaminationResult {
    let result = (scores.0 + scores.1 + scores.2) / 3;

    match result {
        70..=100 => ExaminationResult::Passed(String::from("Passed")),
        _ => ExaminationResult::NotPassed(String::from("Not passed")),
    }
}

fn check_result(state: &ExaminationResult) {
    match state {
        ExaminationResult::Passed(p) => println!("{}", p),
        ExaminationResult::NotPassed(f) => println!("{}", f),
    }
}

pub fn enums_print() {
    let student_score_1 = (80, 80, 70);

    let student_score_state = determine_student_result(student_score_1);

    check_result(&student_score_state);
}
