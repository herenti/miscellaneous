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
        let num = 100.0/count as f64;
        scores.push((num * 100.0).round() / 100.0);
    }
    let mut counts = HashMap::new();

    for &x in &choices {
        *counts.entry(x).or_insert(0) += 1;
    }
    let max_count = counts.values().copied().max().unwrap();
    let score = scores[0] + scores[1] + scores[2];
    match max_count {
        1 => {
            println!("{}|{}|{} -- You win: ${}", choice1, choice2, choice3, score);
        },
        2 => {
            let score = score*2.0;
            println!("{}|{}|{} -- You win: ${} -- Two of a kind!", choice1, choice2, choice3, score);
        },
        3 => {
            if score == 300.0 {
                let score = score*1000.0;
                println!("{}|{}|{} -- You win: ${} -- Three of a kind! Jackpot!", choice1, choice2, choice3, score);
            } else {
                let score = score*3.0;
                println!("{}|{}|{} -- You win: ${} -- Three of a kind!", choice1, choice2, choice3, score);
            }
        },
        _ => {
        }
    }
}
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
        println!("{}", count);
        let num = 100.0/count as f64;
        scores.push((num * 100.0).round() / 100.0);
    }

    let mut counts = HashMap::new();

    for &x in &choices {
        *counts.entry(x).or_insert(0) += 1;
    }
    let max_count = counts.values().copied().max().unwrap();
    let score = scores[0] + scores[1] + scores[2];
    match max_count {

        1 => {
            println!("{}|{}|{} -- You win: ${}", choice1, choice2, choice3, score);
        },
        2 => {
            let score = score*2.0;
            println!("{}|{}|{} -- You win: ${} -- Two of a kind!", choice1, choice2, choice3, score);
        },
        3 => {
            if score == 300.0 {
                let score = score*1000.0;
                println!("{}|{}|{} -- You win: ${} -- Three of a kind! Jackpot!", choice1, choice2, choice3, score);
            } else {
                let score = score*3.0;
                println!("{}|{}|{} -- You win: ${} -- Three of a kind!", choice1, choice2, choice3, score);
            }
        },
        _ => {

        }
    }



}
