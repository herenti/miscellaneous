use rand::prelude::IndexedRandom;
use std::io::Write;
use std::io;



fn main() {
    let mut running = true;
    let mut questions = vec![];
    if let Ok(contents) = std::fs::read_to_string("questions.txt"){
        let contents = contents.lines();
        let contents = contents.collect::<Vec<&str>>();
        for i in contents {
            let line = i.trim();
            let line = line.split(":");
            let line = line.collect::<Vec<&str>>();
            if line.len() < 1 {
                println!("\r\n The questions.txt file is empty. Please see the readme for info. No questions to begin.\r\n");
                running = false;
            }
            let question = line[0];
            let answer = line[1];
            questions.push((question.to_string(), answer.to_string()));
        }
    }
    while running {
        let mut choices = vec![];
        let mut _questions = questions.clone();
        let mut rng = rand::rng();
        let max_questions = if questions.len() > 4 {
            4
        } else {
            questions.len()
        };
        for i in 0..max_questions {

            let choice = _questions.choose(&mut rng).unwrap().clone();
            let index = _questions.iter().position(|x| *x == choice).unwrap();
            _questions.remove(index);
            choices.push((i.to_string(), choice.0.clone(), choice.1.clone()));
        }
        if choices.len() == 0 {
            println!("All questions answered correctly!");
            running = false;
            break;
        }
        let question = choices.choose(&mut rng).unwrap();
        let q = &question.1;
        let a = &question.2;
        let n = &question.0;
        let question_index = questions.iter().position(|x| *x == (q.to_string(), a.to_string())).unwrap();
        let mut final_string = format!("\r\nQuestion: {}\r\nAnswers:", q);
        for i in 0..choices.len() {
            final_string += &format!("\r\n{}: {}", choices[i].0, choices[i].2);
        }
        println!("{}", final_string);
        println!("\r\nEnter the number of the correct answer and press enter...\r\n");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.\r\n");
        let input = input.trim();
        let input = input.to_lowercase();
        let input = input.as_str();
        if input == n {
            println!("\r\nThat is correct!!! Press any button to continue...");
            questions.remove(question_index);
            std::io::stdout().flush().unwrap();
        }
        else {
            println!("\r\nThat is incorrect. Press any button to continue...");
            std::io::stdout().flush().unwrap();
        }
    }
}
