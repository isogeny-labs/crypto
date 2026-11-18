use std::{collections::BTreeMap, fs};
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let raw = fs::read_to_string(&args[1]).unwrap();
    let outcomes = fs::read_to_string(&args[2]).unwrap();
    let mut meta = BTreeMap::new();
    for l in outcomes.lines().skip(1) {
        let v: Vec<_> = l.split('\t').map(str::to_string).collect();
        assert_eq!(v.len(), 17);
        assert_eq!(v[16], "true");
        meta.entry((v[0].parse::<usize>().unwrap(), v[1].parse::<u64>().unwrap()))
            .or_insert((v[2].clone(), v[3].clone(), v[4].clone(), v[5].clone()));
    }
    assert_eq!(meta.len(), 6);
    let mut rows = BTreeMap::new();
    for l in raw.lines().skip(1) {
        let v: Vec<_> = l.split('\t').collect();
        assert_eq!(v.len(), 24);
        if v[5] != "0" {
            continue;
        }
        let key = (
            v[0].parse::<usize>().unwrap(),
            v[1].parse::<u64>().unwrap(),
            v[2].parse::<u64>().unwrap(),
            v[3].parse::<usize>().unwrap(),
            v[4].to_string(),
            if v[1] == v[6] { "p" } else { "ell" },
        );
        let c: Vec<u64> = v[8..].iter().map(|x| x.parse().unwrap()).collect();
        assert!(rows.insert(key, c).is_none())
    }
    assert_eq!(rows.len(), 192);
    println!("g\tp\tsm\tcase\tmodulus\tcounter\tbatch\tpencil\tfolded\texpanded\tpencil_over_batch\tfolded_over_batch\texpanded_over_batch");
    for ((g, p), (c, ell, m, r)) in meta {
        let mut reductions = vec![];
        let mut inversions = vec![];
        for sm in [1u64, 17] {
            for case in 0..2 {
                for modulus in ["p", "ell"] {
                    let key = |arm: &str| (g, p, sm, case, arm.to_string(), modulus);
                    let b = &rows[&key("batch")];
                    let s = &rows[&key("pencil")];
                    let f = &rows[&key("folded")];
                    let e = &rows[&key("expanded")];
                    if modulus == "p" {
                        reductions.push(f[8] as f64 / b[8] as f64);
                        inversions.push(f[9] as f64 / b[9] as f64);
                        let saved = s[9] - b[9];
                        assert_eq!(b[4] - s[4], 3 * saved);
                        assert_eq!(b[8] - s[8], 6 * saved)
                    } else {
                        assert_eq!(b, s)
                    }
                    for (j, n) in [
                        (4, "mul"),
                        (8, "redc"),
                        (9, "inv"),
                        (10, "inv_steps"),
                        (11, "poly_products"),
                    ] {
                        let ratio = |v: u64| {
                            if b[j] == 0 {
                                "NA".into()
                            } else {
                                format!("{:.6}", v as f64 / b[j] as f64)
                            }
                        };
                        println!(
                            "{g}\t{p}\t{sm}\t{case}\t{modulus}\t{n}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                            b[j],
                            s[j],
                            f[j],
                            e[j],
                            ratio(s[j]),
                            ratio(f[j]),
                            ratio(e[j])
                        );
                    }
                }
            }
        }
        let mm = |v: &[f64]| {
            (
                v.iter().copied().fold(f64::INFINITY, f64::min),
                v.iter().copied().fold(0.0, f64::max),
            )
        };
        eprintln!("g={g} p={p} c={c} ell={ell} m={m} orbits={r} folded_over_batch_redc={:?} folded_over_batch_inv={:?}",mm(&reductions),mm(&inversions));
    }
}
