use std::{collections::BTreeMap, fs};
fn median(x: &mut [f64]) -> f64 {
    x.sort_by(|a, b| a.total_cmp(b));
    x[x.len() / 2]
}
fn main() {
    let raw = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut cells: BTreeMap<(usize, String, usize), Vec<u64>> = BTreeMap::new();
    let mut n = 0;
    for line in raw.lines().skip(1) {
        let v: Vec<_> = line.split('\t').collect();
        assert_eq!(v.len(), 7);
        assert_eq!(v[6], "true");
        cells
            .entry((v[1].parse().unwrap(), v[3].into(), v[0].parse().unwrap()))
            .or_default()
            .push(v[4].parse().unwrap());
        n += 1;
    }
    assert_eq!(n, 7200);
    assert_eq!(cells.len(), 450);
    let mut summaries = BTreeMap::new();
    let mut spread_pass = true;
    println!("workload\tarm\tmedian_batch_ns\tmin_batch_ns\tmax_batch_ns\tspread");
    for id in 0..18 {
        for arm in ["folded", "expanded", "plain_prefix", "native", "native_aa"] {
            let mut means = vec![];
            for b in 0..5 {
                let v = &cells[&(id, arm.into(), b)];
                assert_eq!(v.len(), 16);
                means.push(v.iter().map(|&x| x as f64).sum::<f64>() / 16.0)
            }
            let med = median(&mut means);
            let lo = means[0];
            let hi = means[4];
            spread_pass &= hi / lo <= 1.25;
            println!("{id}\t{arm}\t{med:.3}\t{lo:.3}\t{hi:.3}\t{:.6}", hi / lo);
            summaries.insert((id, arm.to_string()), med);
        }
    }
    let (mut aa_pass, mut tenfold) = (true, true);
    for id in 0..18 {
        let f = summaries[&(id, "folded".into())];
        let e = summaries[&(id, "expanded".into())];
        let p = summaries[&(id, "plain_prefix".into())];
        let n = summaries[&(id, "native".into())];
        let aa = summaries[&(id, "native_aa".into())];
        let ratio = n / aa;
        aa_pass &= (0.90..=1.10).contains(&ratio);
        let best = e.min(p).min(n).min(aa);
        tenfold &= best / f >= 10.0;
        eprintln!("workload={id} expanded_over_folded={:.6} plain_over_folded={:.6} native_over_folded={:.6} best_over_folded={:.6} native_aa={ratio:.6}",e/f,p/f,n/f,best/f);
    }
    eprintln!("calls=7200 correct=true spread_pass={spread_pass} aa_pass={aa_pass} numerical_tenfold={tenfold} local_qualified={}",spread_pass&&aa_pass);
}
