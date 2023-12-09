use self::brain::*;

mod brain;

use rand::seq::SliceRandom;
use std::collections::HashMap;

fn main() {
    let mut brain = Brain::default();

    brain.train(include_str!("../chatgpt.txt"));

    println!("{}", brain.prompt("ChatGPT is a", 64));
}
