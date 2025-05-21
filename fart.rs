use rand::Rng;

fn fart() -> String{
    let choices = ["thbt","ttbbbtt", "tttttthhhbbbbbttt", "flubflublbblb","brrrrrraapppp","brrrurnnnntttt", " ", "shhhhppplaaatt", "surplat", "bromp"];
    let schoices = ["I feel a fart coming on... ", "I am so gassy... ", "Oh nooo im gonna fart... ", "I hope this is not a shart... ", "Please excuse my gas... ", "This might be a stinky one.... ", "Welcome to the fartfest... "];
    let mut rng = rand::thread_rng();
    let rangenum = rng.gen_range(1..4);
    let mut _final = "".to_string();
    for i in 0..rangenum{
        let num = rng.gen_range(0..choices.len());
        _final.push_str(choices[num]);
    };
    let num = rng.gen_range(0..schoices.len());
    let _final = if _final.chars().all(|c| c == ' ') {
        "It was a silent one"
    } else{
        &_final
    };
    format!("{}{}", schoices[num], _final)
}

fn main(){

    println!("{}", fart());

}
