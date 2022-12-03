use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn get_input(filename: &str) -> io::Lines<BufReader<File>> { 
    let path = Path::new(filename);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", display, why),
        Ok(file) => file,
    };
    io::BufReader::new(file).lines()
}

fn first() -> HashMap<(&'static str, &'static str), i32> {
    let mut scores: HashMap<(&str, &str), i32> = HashMap::new();
    scores.insert(("A", "X"), 1 + 3);
    scores.insert(("A", "Y"), 2 + 6);
    scores.insert(("A", "Z"), 3 + 0);
    scores.insert(("B", "X"), 1 + 0);
    scores.insert(("B", "Y"), 2 + 3);
    scores.insert(("B", "Z"), 3 + 6);
    scores.insert(("C", "X"), 1 + 6);
    scores.insert(("C", "Y"), 2 + 0);
    scores.insert(("C", "Z"), 3 + 3);
    scores
}

fn second() -> HashMap<(&'static str, &'static str), i32> {
    let mut scores: HashMap<(&str, &str), i32> = HashMap::new();
    scores.insert(("A", "X"), 3 + 0);
    scores.insert(("A", "Y"), 1 + 3);
    scores.insert(("A", "Z"), 2 + 6);
    scores.insert(("B", "X"), 1 + 0);
    scores.insert(("B", "Y"), 2 + 3);
    scores.insert(("B", "Z"), 3 + 6);
    scores.insert(("C", "X"), 2 + 0);
    scores.insert(("C", "Y"), 3 + 3);
    scores.insert(("C", "Z"), 1 + 6);
    scores
}


fn main() {
    let mut total: i32 = 0;
    let points = second();
    for line in get_input("input") {
        if let Ok(play) = line {
            let mut letter = play.split(" ");
            let a = letter.next().unwrap();
            let b = letter.next().unwrap();
            total += points.get(&(a, b)).unwrap();
        }
    }
    println!("{}", total);
    // println!("A Y: {:?}", points.get(&("A", "Y")));
    // println!("B X: {:?}", points.get(&("B", "X")));
    // println!("C Z: {:?}", points.get(&("C", "Z")));
}
