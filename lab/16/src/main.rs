use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;

fn main() {
    // check if debug
    let args: Vec<String> = env::args().collect();
    let debug = if args.len() > 1 && args[1] == "debug" { true } else { false };
    // find test files
    let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filepath.push("resources/");
    filepath.push(if debug { "debug.txt" } else { "rosalind_ba4a.txt" });

    let mut f = File::open(filepath).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);

    let lines_iter = data.lines();

    for line in lines_iter {
        let mut output = String::new();
        let mut chars = line.chars();
        let count = line.chars().count() / 3;
        for _ in 0..count {
            let mut codon = String::new();
            codon.push(chars.next().unwrap());
            codon.push(chars.next().unwrap());
            codon.push(chars.next().unwrap());
            output.push(get_amino_acid(&codon));
        }
        println!("{}", output);
    }
}

fn get_amino_acid(codon: &str) -> char {
    match codon {
        "AAA" => 'K',
        "AAC" => 'N',
        "AAG" => 'K',
        "AAU" => 'N',
        "ACA" => 'T',
        "ACC" => 'T',
        "ACG" => 'T',
        "ACU" => 'T',
        "AGA" => 'R',
        "AGC" => 'S',
        "AGG" => 'R',
        "AGU" => 'S',
        "AUA" => 'I',
        "AUC" => 'I',
        "AUG" => 'M',
        "AUU" => 'I',
        "CAA" => 'Q',
        "CAC" => 'H',
        "CAG" => 'Q',
        "CAU" => 'H',
        "CCA" => 'P',
        "CCC" => 'P',
        "CCG" => 'P',
        "CCU" => 'P',
        "CGA" => 'R',
        "CGC" => 'R',
        "CGG" => 'R',
        "CGU" => 'R',
        "CUA" => 'L',
        "CUC" => 'L',
        "CUG" => 'L',
        "CUU" => 'L',
        "GAA" => 'E',
        "GAC" => 'D',
        "GAG" => 'E',
        "GAU" => 'D',
        "GCA" => 'A',
        "GCC" => 'A',
        "GCG" => 'A',
        "GCU" => 'A',
        "GGA" => 'G',
        "GGC" => 'G',
        "GGG" => 'G',
        "GGU" => 'G',
        "GUA" => 'V',
        "GUC" => 'V',
        "GUG" => 'V',
        "GUU" => 'V',
        "UAA" => '*',
        "UAC" => 'Y',
        "UAG" => '*',
        "UAU" => 'Y',
        "UCA" => 'S',
        "UCC" => 'S',
        "UCG" => 'S',
        "UCU" => 'S',
        "UGA" => '*',
        "UGC" => 'C',
        "UGG" => 'W',
        "UGU" => 'C',
        "UUA" => 'L',
        "UUC" => 'F',
        "UUG" => 'L',
        "UUU" => 'F',
        _ => '0',
    }
}