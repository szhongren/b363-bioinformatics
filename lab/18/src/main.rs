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
    filepath.push(if debug { "debug.txt" } else { "rosalind_ba5c.txt" });

    let mut f = File::open(filepath).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);
    let count = if debug { 2 } else { 1 };
    let mut lines_iter = data.lines();
    for i in 0..count {
        let str0 = lines_iter.next().unwrap();
        let str1 = lines_iter.next().unwrap();
        println!("{}-{}", str0, str1);
    }
}
