use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;

fn main() {
    // check if debug
    let args: Vec<String> = env::args().collect();
    let debug = if args.len() > 1 && args[1] == "debug" { true } else { false };
    let debug1 = if args.len() > 1 && args[1] == "debug1" { true } else { false };
    // find test files
    let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filepath.push("resources/");
    filepath.push(if debug { "debug.txt" } else if debug1 { "debug1.txt" } else { "rosalind_ba3e.txt" });

    let mut f = File::open(filepath).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);

    let lines_iter = data.lines();
    let mut edges = Vec::<String>::new();

    for line in lines_iter {
        let count = line.clone().chars().count();
        edges.push(line.to_string());
    }
}