use itertools::Itertools;                                                                                              

fn main() {
    let input = include_str!("../input");
    let mut full_overlap = 0;
    let mut part_overlap = 0;
    for line in input.lines() {
        let elfpair = line.split(',');
        
        let mut r: Vec<i32> = Vec::new();
        let mut l: Vec<usize> = Vec::new();
        for elf in elfpair {
            let mut sections = elf.split('-');
            let a = sections.next().unwrap().parse::<i32>().unwrap();
            let b = sections.next().unwrap().parse::<i32>().unwrap();
            let mut c = (a..b+1).collect_vec();
            l.push(c.len());
            r.append(&mut c);
        }
        let v = r.into_iter().duplicates().collect_vec();
        if l.contains(&v.len()){ full_overlap += 1 };
        if v.len() > 0 { part_overlap += 1 };
    }
    println!("{}", full_overlap);
    println!("{}", part_overlap);
}
