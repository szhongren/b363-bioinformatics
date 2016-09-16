use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

static NUCLEOTIDES: [u8; 4] = [b'A', b'C', b'G', b'T'];

fn main() {
    let debug = true;
    let filename = if debug { "debug.txt" } else { "rosalind_ba1j.txt" };
    let sets = if debug { 7 } else { 1 };

    let mut data = String::new();
    let mut f = File::open(filename).expect("Open file");
    f.read_to_string(&mut data).expect("Parse into string");

    let mut lines_iter = data.lines();
    for _ in 0..sets {
        let (genome, k, d) = read_data(&mut lines_iter);
        for i in freq_words_with_mismatches_complements(genome.as_bytes(), k, d) {
            print!("{} ", String::from_utf8(i).unwrap());
        }
        print!("\n");
    }
}

fn read_data(lines_iter: &mut std::str::Lines) -> (String, i32, i32) {
    let genome = lines_iter.next().expect("Read genome").to_string();
    let mut k_d = lines_iter.next().expect("Read k, d").split_whitespace();
    let k = k_d.next().expect("Parse k").parse::<i32>().unwrap();
    let d = k_d.next().expect("Parse d").parse::<i32>().unwrap();
    (genome, k, d)
}

fn hamming_distance(a: &[u8], b: &[u8]) -> i32 {
    assert_eq!(a.len(), b.len());
    let mut distance = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            distance += 1;
        }
    }
    distance
}

fn neighbors(pattern: &[u8], d: i32) -> Vec<Vec<u8>> {
    if d == 0 { return vec![Vec::from(pattern)]; }
    if pattern.len() == 1 {
        return NUCLEOTIDES.iter().map(|n| vec![*n]).collect::<Vec<_>>();
    }

    let pattern_suffix = &pattern[1..];
    let suffix_neighbors = neighbors(&pattern_suffix, d);
    let mut neighborhood: Vec<Vec<u8>> = Vec::new();
    for suffix_neighbor in suffix_neighbors {
        if hamming_distance(&pattern_suffix, suffix_neighbor.as_slice()) < d {
            for nucleotide in NUCLEOTIDES.iter() {
                let mut new_neighbor: Vec<u8> = Vec::with_capacity(1 + suffix_neighbor.len());
                new_neighbor.push(*nucleotide);
                new_neighbor.extend(&suffix_neighbor);
                neighborhood.push(new_neighbor);
            }
        } else {
            let mut new_neighbor: Vec<u8> = Vec::with_capacity(1 + suffix_neighbor.len());
            new_neighbor.push(pattern[0]);
            new_neighbor.extend(&suffix_neighbor);
            neighborhood.push(new_neighbor);
        }
    }
    neighborhood
}

#[allow(dead_code)]
fn freq_words_with_mismatches(genome: &[u8], k: i32, d: i32) -> Vec<Vec<u8>> {

    let k_usize = k as usize;
    let mut freq_patterns: Vec<Vec<u8>> = Vec::new();
    let mut neighborhoods: Vec<Vec<u8>> = Vec::new();
    for i in 0..(genome.len() - k_usize + 1) {
        neighborhoods.append(&mut neighbors(&genome[i..i+k_usize], d));
    }
    neighborhoods.sort();

    let mut max_count = 0;
    let mut count = 0;
    let mut curr = Vec::new();

    for i in 0..neighborhoods.len() {
        // println!("{:?}, c = {}, mc = {}", String::from_utf8(neighborhoods[i].clone()).unwrap(), count, max_count);
        if neighborhoods[i] == curr {
            count += 1;
            continue;
        } else {
            if count > max_count {
                freq_patterns = Vec::new();
                max_count = count;
            }
            if curr.is_empty() == false && count >= max_count {
                freq_patterns.push(curr);
            }
            count = 1;
            curr = neighborhoods[i].clone();
        }
    }

    if count > max_count {
        freq_patterns = Vec::new();
        max_count = count;
    }
    if curr.is_empty() == false && count >= max_count {
        freq_patterns.push(curr);
    }

    freq_patterns
}

fn freq_words_with_mismatches_complements(genome: &[u8], k: i32, d: i32) -> Vec<Vec<u8>> {

    let k_usize = k as usize;
    let mut freq_patterns: Vec<Vec<u8>> = Vec::new();
    let mut neighborhoods: Vec<Vec<u8>> = Vec::new();
    let mut freq_array = HashMap::new();

    for i in 0..(genome.len() - k_usize + 1) {
        neighborhoods.append(&mut neighbors(&genome[i..i+k_usize], d));
    }
    neighborhoods.sort();

    let mut count = 0;
    let mut curr = Vec::new();

    for i in 0..neighborhoods.len() {
        if neighborhoods[i] == curr {
            count += 1;
            continue;
        } else {
            if curr.is_empty() == false {
                freq_array.insert(curr, count);
            }
            count = 1;
            curr = neighborhoods[i].clone();
        }
    }
    freq_array.insert(curr, count);

    let freq_array2 = freq_array.clone();
    for (neighbor, freq) in freq_array2 {
        let complement = reverse_complement(String::from_utf8(neighbor).unwrap().as_bytes());
        let freq1: &mut i32 = freq_array.entry(complement).or_insert(0);
        *freq1 += freq;
        // println!("{}: {} -> {}", String::from_utf8(neighbor).unwrap(), freq, String::from_utf8(complement).unwrap());
    }

    let mut max_count = 0;
    for (neighbor, freq) in freq_array {
        if freq > max_count {
            freq_patterns = Vec::new();
            freq_patterns.push(neighbor);
            max_count = freq;
        } else if freq == max_count {
            freq_patterns.push(neighbor);
        }
    }
    freq_patterns
}

#[allow(dead_code)]
fn pattern_to_number(pattern: &[u8]) -> i32 {
    let mut sum = 0;
    for i in 0..pattern.len() {
        let curr = match pattern[i] as char {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => -1,
        };
        sum = sum * 4 + curr;
    }
    sum
}

#[allow(dead_code)]
fn number_to_pattern(n: i32, k: i32) -> Vec<u8> {
    let mut pattern = Vec::new();
    let mut stack = Vec::new();
    let mut quotient = n;
    for _ in 0..k {
        let curr = match quotient % 4 {
            0 => 'A',
            1 => 'C',
            2 => 'G',
            3 => 'T',
            _ => '\0',
        };
        stack.push(curr as u8);
        quotient /= 4;
    }
    while stack.is_empty() == false {
        pattern.push(stack.pop().expect("Reversing the order"));
    }
    pattern
}

fn reverse_complement(pattern: &[u8]) -> Vec<u8> {
    let mut reverse = Vec::new();
    let mut original = Vec::from(pattern);
    while original.is_empty() == false {
        reverse.push(match original.pop().expect("Reverse complement") as char {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => '\0',
        } as u8);
    }
    reverse
}