use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug)]
struct Aacid {
    short: &'static str,
    mass: i32,
}

impl fmt::Display for Aacid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.short, self.mass)
    }
}

static AACIDS: [Aacid; 18] =
    [Aacid{ short: "G" , mass: 57 },
    Aacid{ short: "A" , mass: 71 },
    Aacid{ short: "S" , mass: 87 },
    Aacid{ short: "P" , mass: 97 },
    Aacid{ short: "V" , mass: 99 },
    Aacid{ short: "T" , mass: 101 },
    Aacid{ short: "C" , mass: 103 },
    // Aacid{ short: "I" , mass: 113 },
    Aacid{ short: "(I/L)" , mass: 113 },
    Aacid{ short: "N" , mass: 114 },
    Aacid{ short: "D" , mass: 115 },
    // Aacid{ short: "K" , mass: 128 },
    Aacid{ short: "(K/Q)" , mass: 128 },
    Aacid{ short: "E" , mass: 129 },
    Aacid{ short: "M" , mass: 131 },
    Aacid{ short: "H" , mass: 137 },
    Aacid{ short: "F" , mass: 147 },
    Aacid{ short: "R" , mass: 156 },
    Aacid{ short: "Y" , mass: 163 },
    Aacid{ short: "W" , mass: 186 }];

fn main() {
    // check if debug
    let args: Vec<String> = env::args().collect();
    let debug = if args.len() > 1 && args[1] == "debug" { true } else { false };
    // find test files
    let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filepath.push("resources/");
    filepath.push(if debug { "debug.txt" } else { "rosalind_ba4e.txt" });

    let mut f = File::open(filepath).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);

    let mut lines_iter = data.lines();
    for line in lines_iter {
        let spec = read_data(line.to_string());
        let res = cyclopeptide_seq(&spec);
        print_ans(res);
    }
}

fn read_data(line: String) -> HashMap<i32, i32> {
    let spec: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut ans = HashMap::new();
    for i in spec {
        let count = ans.entry(i).or_insert(0);
        *count += 1;
    }
    ans
}

fn print_ans(ans: Vec<Vec<Aacid>>) {
    let l = ans[0].len();
    for peptide in ans {
        for i in 1..l {
            print!("{}", peptide[i].mass);
            if i == l - 1 {
                break;
            }
            print!("-");
        }
        print!(" ");
    }
    print!("\n");
}

fn cyclopeptide_seq(spec: &HashMap<i32, i32>) -> Vec<Vec<Aacid>> {
    let mut peptides = vec![vec![Aacid{ short: "0", mass: 0 }]];
    let max_mass = parent_mass(spec);
    let mut ans = Vec::new();
    while !peptides.is_empty() {
        let mut newpeps = Vec::new();
        peptides = expand(peptides);
        for peptide in peptides {
            if mass(&peptide) == max_mass {
                if &cyclospectrum(&peptide) == spec {
                    ans.push(peptide);
                }
            } else if consistent(linear_spectrum(&peptide), spec) {
                newpeps.push(peptide);
            }
        }
        peptides = newpeps;
    }
    ans
}

fn expand(peptides: Vec<Vec<Aacid>>) -> Vec<Vec<Aacid>> {
    let mut new: Vec<Vec<Aacid>> = Vec::new();
    for peptide in peptides {
        for &aa in AACIDS.iter() {
            let mut newpep = peptide.clone();
            newpep.push(aa);
            new.push(newpep);
        }
    }
    new
}

fn parent_mass(spec: &HashMap<i32, i32>) -> i32 {
    let mut mass: i32 = 0;
    for &i in spec.keys() {
        if i > mass {
            mass = i;
        }
    }
    mass
}

fn mass(peptide: &Vec<Aacid>) -> i32 {
    let mut sum = 0;
    for aa in peptide {
        sum += aa.mass;
    }
    sum
}

fn consistent(lin_spec: HashMap<i32, i32>, spec: &HashMap<i32, i32>) -> bool {
    for mass in lin_spec.keys() {
        if spec.contains_key(mass) {
            if lin_spec.get(mass).unwrap() > spec.get(mass).unwrap() {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn sub_sum(peptide: &Vec<Aacid>, i: usize, j: usize) -> i32 {
    let mut sum = 0;
    for x in i..j + 1 {
        sum += peptide[x].mass;
    }
    sum
}

fn linear_spectrum(peptide: &Vec<Aacid>) -> HashMap<i32, i32> {
    let mut spec = HashMap::new();
    spec.insert(0, 1);
    let l = peptide.len();
    for i in 1..l {
        for j in i..l {
            let sum = sub_sum(peptide, i, j);
            let count = spec.entry(sum).or_insert(0);
            *count += 1;
        }
    }
    spec
}

fn cyclospectrum(peptide: &Vec<Aacid>) -> HashMap<i32, i32> {
    let mut spec = linear_spectrum(peptide);
    let l = peptide.len();
    if l <= 2 {
        return spec;
    }
    for start in 3..l {
        for end in 1..start-1 {
            let cyclosum = sub_sum(peptide, 1, end) + sub_sum(peptide, start, l - 1);
            let count = spec.entry(cyclosum).or_insert(0);
            *count += 1;
        }
    }
    spec
}