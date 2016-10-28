use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;

fn main() {
    // check if debug
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { panic!("File not found") };
    // find test files
    let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filepath.push("resources/");
    filepath.push(args[1].clone());

    let mut f = File::open(filepath).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);

    let mut lines_iter = data.lines();

    let strings = get_data(&mut lines_iter);
    let k = 10;
    let seq_len = 500;
    let d = if args[1] == "cha1.txt" { 3 } else { 4 };
    let best_kmer = naive_find_motif(&strings, k, d, seq_len);
    let best_indices = find_best_indices(best_kmer);

}

fn get_data(lines_iter: &mut std::str::Lines) -> Vec<String> {
    let mut data = Vec::new();
    let mut buf = Vec::new();
    for (i, line) in lines_iter.enumerate() {
        if i % 9 != 0 {
            buf.push(line.to_string());
        }
    }
    let mut strand = String::new();
    for (i, line) in buf.into_iter().enumerate() {
        strand = format!("{}{}", strand, line);
        if i % 8 == 7 {
            data.push(strand.clone());
            strand = String::new();
        }
    }
    data
}

fn find_best_indices(best_kmer: String) -> Vec<i32> {
    println!("{}", best_kmer);
    Vec::new()
}

fn naive_find_motif(strings: &Vec<String>, k: i32, d: i32, seq_len: usize) -> String {
    //! guaranteed to work, but slow since it runs in O(4^ktn) time
    let all_potential = gen_all_kmers(k);
    let mut best_profile = i32::max_value();
    let mut best_kmer = String::new();
    let mut total = 1;
    for _ in 0..k {
        total *= 4;
    }
    for (i, kmer) in all_potential.into_iter().enumerate() {
        println!("{}/{} - {}", i, total, kmer);
        let mut profile = 0;
        for seq in strings.clone() {
            let mut best_hamming = k + 2;
            for i in 0..seq_len - k as usize + 1 {
                let hamming = hamming_distance_with_d(&seq[i..i+k as usize], &kmer, d, k);
                if hamming < best_hamming {
                    best_hamming = hamming;
                    // println!("{} - {}[{}], {}", &kmer, seq, i, best_hamming);
                }
            }
            profile += best_hamming;
            if profile >= best_profile {
                continue;
            }
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
    let nucleotides = ["A", "C", "G", "T"].iter().map(|x| x.to_string()).collect();
    if k == 0 {
        return output;
    } else if k == 1 {
        return nucleotides;
    } else {
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

fn hamming_distance_with_d(a: &str, b: &str, d: i32, k: i32) -> i32 {
    let mut distance = 0;
    for (x, y) in a.chars().zip(b.chars()) {
        if x != y {
            distance += 1;
        }
        if distance >= d {
            return k + 1;
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