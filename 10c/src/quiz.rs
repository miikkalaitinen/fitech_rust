use crate::util::read_input;
use rand::seq::SliceRandom;
use rand::thread_rng;


pub fn ask_question(question: &str, correct_answer: &str) -> bool {
    println!("{}", question);
    let user_answer = read_input();
    user_answer == correct_answer
}

pub fn practice_problems(problems: &[(&str, &str)]) {
    let mut score = 0;
    for (question, correct_answer) in problems {
        let correct = ask_question(question, correct_answer);
        if correct {
            score += 1;
            println!("Correct!");
        } else {
            println!("Incorrect! The correct answer is {}", correct_answer);
        }
    }
    println!("You got {} out of {} correct!", score, problems.len());
}

pub fn practice_problems_random_order(problems: &[(&str, &str)]) {

    let mut rng = thread_rng();
    let mut shuffle_problems = problems.to_vec();
    println!("Unshuffled: {:?}", shuffle_problems);
    shuffle_problems.shuffle(&mut rng);
    println!("Shuffled:   {:?}", shuffle_problems);
    practice_problems(&shuffle_problems);
}
