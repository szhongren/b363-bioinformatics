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
    filepath.push(if debug { "debug.txt" } else { "rosalind_ba3b.txt" });
    let sets = if debug { 7 } else { 1 };

    let mut f = File::open(filepath).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);

    let mut lines_iter = data.lines();
    let count = lines_iter.clone().count();

    for (i, line) in lines_iter.enumerate() {
        if i <  count - 1 {
            print!("{}", line.to_string().as_bytes()[0] as char);
        } else {
            print!("{}", line.to_string());
        }
    }
}