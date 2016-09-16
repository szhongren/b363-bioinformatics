use std::fs::File;
use std::io::Read;
use std::str::Chars;

fn main() {
    let DEBUG = true;
    let filename = if DEBUG { "debug.txt" } else { "rosalind_ba1g.txt" };
    let sets = if DEBUG { 6 } else { 1 };

    let mut f = File::open(filename).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);

    let mut lines_iter = data.lines();

    for _ in 0..sets {
        let genome1 = lines_iter.next().unwrap().to_string();
        let genome2 = lines_iter.next().unwrap().to_string();
        let hamming = hamming_distance(&genome1, &genome2);
        println!("{}", hamming);
    }
}

fn hamming_distance(genome1: &String, genome2: &String) -> i32 {
    let mut hamming = 0;

    let comp_iter = genome1.chars().zip(genome2.chars());

    for (a, b) in comp_iter {
        if a != b {
            hamming += 1;
        }
    }
    hamming
}