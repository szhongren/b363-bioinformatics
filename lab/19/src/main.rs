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
    filepath.push(if debug { "debug.txt" } else { "rosalind_ba5j.txt" });

    let mut f = File::open(filepath).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);
    let sets = if debug { 2 } else { 1 };

    let mut lines_iter = data.lines();
    for _ in 0..sets {
        let a = lines_iter.next().unwrap().to_string();
        let b = lines_iter.next().unwrap().to_string();
        println!("{}, {}", a, b);
    }
fn score(a: String, b: String) -> i32 {
    return 0;
}