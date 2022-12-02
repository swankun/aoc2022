use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let lines = io::BufReader::new(file).lines();

    let mut v: Vec<i32> = Vec::new();
    let mut s = 0i32;
    for line in lines {
        if let Ok(calories) = line {
            if !calories.is_empty() {
                s += calories.parse::<i32>().unwrap();
            } else {
                v.push(s);
                s = 0i32;
            }
        }
    }
    v.sort();    
    let len = v.len();
    let slice = &v[len-3..len];
    for s in slice {
        println!("{}", s);
    }
    println!("Sum = {}", slice.iter().sum::<i32>());
}

/*
 * 65573
 * 67568
 * 70764
 * Sum = 203905
 */
