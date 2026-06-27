use rand::prelude::IndexedRandom;
use std::collections::HashMap;


fn main() {
    let a = ["★"; 1];
    let b = ["fart"; 5];
    let c = ["waifu"; 10];
    let d = ["cheese"; 50];
    let e = ["trash"; 100];
    let mut f = a.to_vec();
    f.extend(b);
    f.extend(c);
    f.extend(d);
    f.extend(e);
    let mut rng = rand::rng();
    let choice1 = f.choose(&mut rng).unwrap();
    let choice2 = f.choose(&mut rng).unwrap();
    let choice3 = f.choose(&mut rng).unwrap();
    let choices = [&choice1, &choice2, &choice3];
    let mut scores = vec![];
    for i in choices {
        match **i {
            "★" => scores.push(100),
            "fart" => scores.push(50),
            "waifu" => scores.push(10),
            "cheese" => scores.push(5),
            "trash" => scores.push(1),
            _ => (),

        }
    }

    let mut counts = HashMap::new();

    for &x in &scores {
        *counts.entry(x).or_insert(0) += 1;
    }
    let max_count = counts.values().copied().max().unwrap_or(0);
    match max_count {

        1 => {
            let score = scores[0] + scores[1] + scores[2];
            println!("{}|{}|{} -- Score: {}", choice1, choice2, choice3, score);
        },
        2 => {
            let score = scores[0] + scores[1] + scores[2];
            let score = score*2;
            println!("{}|{}|{} -- Score: {} -- Two of a kind!", choice1, choice2, choice3, score);
        },
        3 => {
            let score = scores[0] + scores[1] + scores[2];
            if score == 300 {
                let score = score*1000;
                println!("{}|{}|{} -- Score: {} -- Three of a kind! Jackpot!", choice1, choice2, choice3, score);
            } else {
                let score = score*3;
                println!("{}|{}|{} -- Score: {} -- Three of a kind!", choice1, choice2, choice3, score);
            }
        },
        _ => {

        }
    }



}
