enum ExaminationResult {
    Passed(String),
    NotPassed(String),
}

#[derive(Debug)]
enum AlphabetScore {
    A,
    B,
    C,
    D,
    E,
}

fn determine_student_result(scores: (i32, i32, i32)) -> (ExaminationResult, AlphabetScore) {
    let average = (scores.0 + scores.1 + scores.2) / 3;

    let exam_result = match average {
        70..=100 => ExaminationResult::Passed(String::from("Passed")),
        _ => ExaminationResult::NotPassed(String::from("Not passed")),
    };

    let grade = determine_student_alphabet_result(average);

    (exam_result, grade)
}

fn determine_student_alphabet_result(average: i32) -> AlphabetScore {
    match average {
        10..=49 => AlphabetScore::D,
        50..=74 => AlphabetScore::C,
        75..=84 => AlphabetScore::B,
        85..=100 => AlphabetScore::A,
        _ => AlphabetScore::E,
    }
}

fn print_result(result: ExaminationResult, grade: AlphabetScore) {
    match result {
        ExaminationResult::Passed(msg) => println!("✅ {} Grade: {:?}", msg, grade),
        ExaminationResult::NotPassed(msg) => println!("❌ {} Grade: {:?}", msg, grade),
    }
}

pub fn enums_print() {
    let student_score_1 = (80, 80, 30);

    let (result, grade) = determine_student_result(student_score_1);

    print_result(result, grade);
}
