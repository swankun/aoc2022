use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};
use itertools::Itertools;

fn eachline(filename: &str) -> io::Lines<BufReader<File>> { 
    let path = Path::new(filename);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", display, why),
        Ok(file) => file,
    };
    io::BufReader::new(file).lines()
}

fn find_dup(s1: &String, s2: &String) -> String {
    let s1u = s1.chars().unique();
    let s2u = s2.chars().unique();
    let candidates = s1u.cartesian_product(s2u);
    let dup = candidates.filter(|(a,b)| a == b);
    dup.map(|(a,_)| a.to_string()).collect_vec().concat()
}

fn to_priority(c: &char) -> u32 {
    let mut digit = c.to_lowercase().next().unwrap().to_digit(36).unwrap();
    digit -= 9;
    if c.is_uppercase() {
        digit += 26;
    }
    digit
}

fn main() {
    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for (l1, l2, l3) in eachline("input").tuples() {
        let (s1, s2, s3) = (l1.unwrap(), l2.unwrap(), l3.unwrap());
        // println!("{}", s1);
        // println!("{}", s2);
        // println!("{}", s3);
        
        // Part 1
        for s in [&s1, &s2, &s3] {
            let (front, back) = s.split_at(s.len() / 2);
            let dups = find_dup(&front.to_string(), &back.to_string());
            total1 += to_priority(&dups.chars().next().unwrap());
        }

        // Part 2
        let dups = find_dup(&find_dup(&s1, &s2), &s3);
        total2 += to_priority(&dups.chars().next().unwrap());
        println!("{}", dups)
    }
    println!("Part 1: {}", total1);
    println!("Part 2: {}", total2);
}
