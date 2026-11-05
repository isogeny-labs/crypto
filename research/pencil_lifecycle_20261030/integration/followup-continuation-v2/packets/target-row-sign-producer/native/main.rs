use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("input");
        let (p, data) = line.split_once('|').expect("field and rows");
        let p: u64 = p.parse().expect("modulus");
        let rows: Vec<Vec<u64>> = if data.is_empty() { vec![] } else {
            data.split(';').map(|row| row.split(',').map(|v| v.parse().expect("residue")).collect()).collect()
        };
        let r = pencil_target_information::information(p, 1, &rows).expect("fixed field rows");
        println!("{},{},{},{},{}", r.nuisance_rank, r.nuisance_and_target_rank, r.full_rank, r.consistent, r.target_determined);
    }
}
