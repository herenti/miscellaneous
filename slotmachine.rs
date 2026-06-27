use rand::prelude::IndexedRandom;
use std::collections::HashMap;


fn main() {
    let a = ["★"; 1];
    let b = ["fart"; 5];
    let c = ["waifu"; 10];
    let d = ["cheese"; 50];
    let e = ["trash"; 100];
    let f = ["derp"; 25];
    let g = ["pancake"; 75];
    let mut list = a.to_vec();
    list.extend(b);
    list.extend(c);
    list.extend(d);
    list.extend(e);
    list.extend(f);
    list.extend(g);
    let mut rng = rand::rng();
    let choice1 = list.choose(&mut rng).unwrap();
    let choice2 = list.choose(&mut rng).unwrap();
    let choice3 = list.choose(&mut rng).unwrap();
    let choices = [&choice1, &choice2, &choice3];
    let mut scores = vec![];
    for i in choices {
        let count = list.iter().filter(|&&s| s == **i).count();
        scores.push(100/count)
    }

    let mut counts = HashMap::new();

    for &x in &scores {
        *counts.entry(x).or_insert(0) += 1;
    }
    let max_count = counts.values().copied().max().unwrap();
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
