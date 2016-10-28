extern crate rand;

use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;
use rand::random;
use std::collections::HashMap;

fn main() {
    // check if debug
    let args: Vec<String> = env::args().collect();
    let debug = if args.len() > 1 && args[1] == "debug" { true } else { false };
    // find test files
    let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filepath.push("resources/");
    filepath.push(if debug { "debug.txt" } else { "rosalind_ba2b.txt" });
    let sets = if debug { 7 } else { 1 };

    let mut f = File::open(filepath).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);

    let mut lines_iter = data.lines();

    for _ in 0..sets {
        let (k, strings) = get_data(&mut lines_iter, debug);
        let best_kmer = naive_find_motif(&strings, k);
        println!("{}", best_kmer);
    }
}

fn get_data(lines_iter: &mut std::str::Lines, debug: bool) -> (i32, Vec<String>) {
    let mut k_d = lines_iter.next().expect("Read k_d").split_whitespace();
    let k = k_d.next().expect("Parse k").parse::<i32>().unwrap();
    let mut data = Vec::new();
    if debug {
        let d = k_d.next().expect("Parse d").parse::<i32>().unwrap();
        for _ in 0..d {
            data.push(lines_iter.next().expect("Read string").to_string());
        }
    } else {
        data = lines_iter.map(|x| x.to_string()).collect();
    }
    (k, data)
}

fn naive_find_motif(strings: &Vec<String>, k: i32) -> String {
    //! guaranteed to work, but slow since it runs in O(4^ktn) time
    let all_potential = gen_all_kmers(k);
    let mut best_profile = i32::max_value();
    let mut best_kmer = String::new();
    for kmer in all_potential {
        let mut profile = 0;
        for seq in strings {
            let mut best_hamming = i32::max_value();
            for i in 0..&seq.len() - k as usize + 1 {
                let hamming = hamming_distance(&String::from(&seq[i..i+k as usize]), &kmer);
                if hamming < best_hamming {
                    best_hamming = hamming;
                    // println!("{} - {}[{}], {}", &kmer, seq, i, best_hamming);
                }
            }
            profile += best_hamming;
        }
        if profile < best_profile {
            best_profile = profile;
            best_kmer = kmer;
        }
        // println!("{}", best_profile);

    }
    // println!("{}", best_kmer);
    best_kmer
}

fn gen_all_kmers(k: i32) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    // just converts the literal list of chars into a vector of strings
    let nucleotides = ["A", "C", "G", "T"].iter().map(|x| x.to_string()).collect();
    if k == 0 {
        // return empty list
        return output;
    } else if k == 1 {
        // not needed technically, just returns the list of 1 nucleotides
        return nucleotides;
    } else {
        // else concat each nucleotide to each sequence we have so far
        for suffix in gen_all_kmers(k - 1) {
            for nt in &nucleotides {
                let new = suffix.clone() + &nt;
                output.push(new);
            }
        }
    }
    output
}

fn hamming_distance(a: &String, b: &String) -> i32 {
    let mut distance = 0;
    for (x, y) in a.chars().zip(b.chars()) {
        if x != y {
            distance += 1;
        }
    }
    distance
}

// fn gibbs_sampler(strings: &Vec<String>, k: i32) -> String {
//     let mut indices = Vec::new();
//     let string_count = strings.len();
//     let string_len = strings[0].len();
//     for _ in 0..string_count {
//         indices.push(random::<usize>() % (string_len - k as usize + 1));
//     }

//     let mut last_last_consensus = String::new();
//     let mut last_consensus = String::new();
//     let mut consensus = String::new();
//     loop {
//         last_last_consensus = last_consensus;
//         last_consensus = consensus;
//         consensus = find_consensus(strings, indices);
//         if last_last_consensus == last_consensus && last_consensus == consensus {
//             break;
//         }
//     }
//     // motifs <- randomly select k-mers, 1 from each string
//     // best motifs <- motifs
//     // for n in 0..x
//     //   i <- random(t)
//     //   Profile <- profile matrix from all strings except motif_i
//     //   motif_i <- randomly generated k-mer in the ith sequence from profile
//     //   if score(motifs) < score(best_motifs)
//     //     best_motifs <- motifs
//     // return best_motifs

//     consensus
// }

fn char_to_i(x: char) -> usize {
    match x {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => 255,
    }
}

fn i_to_char(x: usize) -> char {
    match x {
        0 => 'A',
        1 => 'C',
        2 => 'G',
        3 => 'T',
        _ => '\0',
    }
}