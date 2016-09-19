use std::fs::File;
use std::io::Read;

fn main() {
    let DEBUG = false;
    let filename = if DEBUG { "debug.txt" } else { "rosalind_tfsq.txt" };

    let mut f = File::open(filename).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);

    let lines_iter = data.lines();
    let mut i = 0;

    for line in lines_iter {
        match i {
            0 => println!(">{}", &line[1..]),
            1 => println!("{}", line),
            _ => (),
        };
        i = (i + 1) % 4;
    }
}