use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("rosalind_ba1f.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data);
    // open file and put in a string

    let mut lines_iter = data.lines();

    // for _ in 0..5 {
    let genome = lines_iter.next().unwrap();
    let nucleotide_iter = genome.chars();

    let mut skew = 0;
    let mut min_points = Vec::new();
    let mut min_skew = 0;

    for (pos, nucleotide) in nucleotide_iter.enumerate() {
        match nucleotide {
            'C' => skew -= 1,
            'G' => skew += 1,
            _ => {},
        }
        // println!("{}, {}, {}, {}", pos, nucleotide, skew, min_skew);
        if skew < min_skew {
            min_points = Vec::new();
            min_skew = skew;
            min_points.push(pos + 1);
        } else if skew == min_skew {
            min_points.push(pos + 1);
        }
    }
    for pt in min_points {
        print!("{} ", pt);
    }
    print!("\n");
    // }
}
