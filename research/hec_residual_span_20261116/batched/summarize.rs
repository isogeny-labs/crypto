use std::{collections::BTreeMap, fs};
fn main() {
    let s = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut rows = BTreeMap::new();
    for l in s.lines().skip(1) {
        let v: Vec<_> = l.split('\t').collect();
        assert_eq!(v.len(), 24);
        if v[5] != "0" {
            continue;
        }
        let key = (
            v[0].parse::<usize>().unwrap(),
            v[2].parse::<u64>().unwrap(),
            v[3].parse::<usize>().unwrap(),
            v[4].to_string(),
            if v[1] == v[6] { "p" } else { "ell" },
        );
        let c: Vec<u64> = v[8..].iter().map(|x| x.parse().unwrap()).collect();
        assert!(rows.insert(key, c).is_none())
    }
    assert_eq!(rows.len(), 144);
    println!("g\tsm\tcase\tmodulus\tcounter\tbatch\tscalar\tfolded\texpanded\tscalar_over_batch\tfolded_over_batch\texpanded_over_batch");
    for g in 2..=4 {
        let mut savings = vec![];
        let mut invratios = vec![];
        let mut redcratios = vec![];
        for sm in [1u64, 17] {
            for case in 0..3 {
                for m in ["p", "ell"] {
                    let b = &rows[&(g, sm, case, "batch".into(), m)];
                    let p = &rows[&(g, sm, case, "pencil".into(), m)];
                    let f = &rows[&(g, sm, case, "folded".into(), m)];
                    let e = &rows[&(g, sm, case, "expanded".into(), m)];
                    if m == "p" {
                        assert!(p[9] >= b[9]);
                        let saved = p[9] - b[9];
                        assert_eq!(b[4] - p[4], 3 * saved);
                        assert_eq!(b[8] - p[8], 6 * saved);
                        savings.push(saved);
                        invratios.push(f[9] as f64 / b[9] as f64);
                        redcratios.push(f[8] as f64 / b[8] as f64)
                    } else {
                        assert_eq!(b, p)
                    }
                    for (j, n) in [
                        (4, "mul"),
                        (8, "redc"),
                        (9, "inv"),
                        (10, "inv_steps"),
                        (11, "poly_products"),
                        (13, "group_add"),
                    ] {
                        let ratio = |x: u64| {
                            if b[j] == 0 {
                                "NA".into()
                            } else {
                                format!("{:.6}", x as f64 / b[j] as f64)
                            }
                        };
                        println!(
                            "{g}\t{sm}\t{case}\t{m}\t{n}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                            b[j],
                            p[j],
                            f[j],
                            e[j],
                            ratio(p[j]),
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
        eprintln!(
            "g={g} saved_inv_calls={:?} folded_over_batch_inv={:?} folded_over_batch_redc={:?}",
            savings,
            mm(&invratios),
            mm(&redcratios)
        );
    }
}
