use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use nalgebra::{Matrix3, Vector3};

fn get_input(filename: &str) -> io::Lines<BufReader<File>> { 
    let path = Path::new(filename);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", display, why),
        Ok(file) => file,
    };
    io::BufReader::new(file).lines()
}

fn construct_choices() -> HashMap<char, Vector3<i32>> {
    let rock = || Vector3::new(1,0,0);
    let papaer = || Vector3::new(0,1,0);
    let scissor = || Vector3::new(0,0,1);
    let mut choice: HashMap<char, Vector3<i32>> = HashMap::new();
    choice.insert('A', rock());
    choice.insert('B', papaer());
    choice.insert('C', scissor());
    choice.insert('X', rock());
    choice.insert('Y', papaer());
    choice.insert('Z', scissor());
    choice
}

fn tuple_from_delimited_pair(s: String, d: char)  -> (char, char) {
    let mut t = s.split(d).take(2);
    (
        t.next().unwrap().chars().next().unwrap(),
        t.next().unwrap().chars().next().unwrap()
    )
}

fn main() {
    let mut total1: i32 = 0;
    let mut total2: i32 = 0;
    let choices = construct_choices();
    let rule1 = Matrix3::new(
        3,6,0,
        0,3,6,
        6,0,3
    ); // reward for win, lose, or draw
    let rule2 = Matrix3::from_diagonal(&Vector3::new(1,2,3)); // reward for choosing each choice
    let rule3 = Matrix3::new(
        0,3,6,
        0,3,6,
        0,3,6
    ); // reward for win, lose, or draw based on second letter
    
    for line in get_input("input") {
        if let Ok(game) = line {
            let (a, b) = tuple_from_delimited_pair(game, ' ');
            let opponent = choices.get(&a).unwrap();
            let you = choices.get(&b).unwrap();

            // Part 1
            let outcome = opponent.dot(&(rule1 * you));
            let chosen_play = you.dot(&(rule2*you));
            total1 += outcome;
            total1 += chosen_play;

            // Part 2
            let desired_outcome = rule3 * you;
            let play_to_choose = rule1.transpose() * opponent - desired_outcome; 
            total2 += desired_outcome.iter().next().unwrap();
            total2 += play_to_choose.abs().argmin().0 as i32 + 1;
        }
    }
    println!("{}", total1);
    println!("{}", total2);
}
