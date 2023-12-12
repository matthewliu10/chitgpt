use self::brain::*;

mod brain;

use rand::seq::SliceRandom;
use std::{collections::HashMap, iter};

fn main() {
    let mut brain = Brain::default();

    brain.train(include_str!("../crime-and-punishment.txt"));

    println!("{}", brain.prompt("CHAPTER II", 256));
}
