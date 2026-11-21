use std::{collections::BTreeMap, fs};
fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}
fn main() {
    let path = std::env::args().nth(1).unwrap();
    let s = fs::read_to_string(path).unwrap();
    let mut cells: BTreeMap<(usize, String, usize), Vec<u64>> = BTreeMap::new();
    let mut calls = 0;
    for l in s.lines().skip(1) {
        let f: Vec<_> = l.split('\t').collect();
        assert_eq!(f.len(), 7);
        assert_eq!(f[6], "true");
        let batch = f[0].parse::<usize>().unwrap();
        let case = f[1].parse::<usize>().unwrap();
        let arm = f[3].to_string();
        let ns = f[4].parse::<u64>().unwrap();
        cells.entry((case, arm, batch)).or_default().push(ns);
        calls += 1;
    }
    assert_eq!(calls, 1200);
    assert_eq!(cells.len(), 75);
    let mut summaries = BTreeMap::new();
    let mut spreads_pass = true;
    println!("case\tarm\tmedian_batch_ns\tmin_batch_ns\tmax_batch_ns\tspread");
    for case in 0..3 {
        for arm in ["character", "qualified", "orbit", "native", "native_aa"] {
            let mut means = vec![];
            for batch in 0..5 {
                let v = &cells[&(case, arm.to_string(), batch)];
                assert_eq!(v.len(), 16);
                means.push(v.iter().map(|&x| x as f64).sum::<f64>() / 16.0)
            }
            let med = median(&mut means);
            let min = means[0];
            let max = means[4];
            let spread = max / min;
            spreads_pass &= spread <= 1.25;
            println!("{case}\t{arm}\t{med:.3}\t{min:.3}\t{max:.3}\t{spread:.6}");
            summaries.insert((case, arm.to_string()), med);
        }
    }
    let mut aa_pass = true;
    let mut tenfold = true;
    for case in 0..3 {
        let a = summaries[&(case, "qualified".into())];
        let b = summaries[&(case, "character".into())];
        let n = summaries[&(case, "native".into())];
        let aa = summaries[&(case, "native_aa".into())];
        let r = n / aa;
        aa_pass &= (0.90..=1.10).contains(&r);
        tenfold &= n / b >= 10.0;
        eprintln!("case={case} qualified_over_character={:.6} native_over_character={:.6} native_over_aa={r:.6}",a/b,n/b);
    }
    eprintln!("calls={calls} correct=true spread_pass={spreads_pass} aa_pass={aa_pass} numerical_tenfold={tenfold} local_qualified={}",spreads_pass&&aa_pass);
}
