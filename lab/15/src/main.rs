use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

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
    let mut edges = Vec::<(String, String)>::new();

    for line in lines_iter {
        let count = line.clone().chars().count();
        edges.push((line[..count - 1].to_string(), line[1..].to_string()));
    }

    let de_bruijn = de_bruijnize(edges);

    print_graph(de_bruijn);
}

fn de_bruijnize(edges: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut res: HashMap<String, Vec<String>> = HashMap::new();
    for (start, end) in edges {
        if res.contains_key(&start) {
            res.get_mut(&start).unwrap().push(end);
        } else {
            res.insert(start.clone(), Vec::new());
            res.get_mut(&start).unwrap().push(end);
        }
    }
    res
}

fn print_graph(de_bruijn: HashMap<String, Vec<String>>) {
    for (start, ends) in de_bruijn {
        print!("{} -> ", start);
        let count = ends.len();
        for i in 0..count {
            if i == count - 1 {
                print!("{}\n", ends[i]);
            } else {
                print!("{},", ends[i]);
            }
        }
    }
}