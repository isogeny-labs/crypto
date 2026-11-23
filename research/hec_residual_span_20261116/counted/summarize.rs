use std::{collections::BTreeMap, fs};
fn main() {
    let raw = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut cells = BTreeMap::new();
    for l in raw.lines().skip(1) {
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
        let counts: Vec<u64> = v[8..].iter().map(|s| s.parse().unwrap()).collect();
        assert!(cells.insert(key, counts).is_none())
    }
    assert_eq!(cells.len(), 108);
    println!("g\tsm\tcase\tmodulus\tcounter\tpencil\tfolded\texpanded\tfolded_over_pencil\texpanded_over_pencil");
    for g in 2..=4 {
        for sm in [1u64, 17] {
            for case in 0..3 {
                for m in ["p", "ell"] {
                    let p = &cells[&(g, sm, case, "pencil".into(), m)];
                    let f = &cells[&(g, sm, case, "folded".into(), m)];
                    let e = &cells[&(g, sm, case, "expanded".into(), m)];
                    for (j, n) in [
                        (8, "redc"),
                        (9, "inv"),
                        (10, "inv_steps"),
                        (11, "poly_products"),
                        (13, "group_add"),
                        (14, "group_add_place"),
                    ] {
                        let ratio = |x: u64| {
                            if p[j] == 0 {
                                "NA".into()
                            } else {
                                format!("{:.6}", x as f64 / p[j] as f64)
                            }
                        };
                        println!(
                            "{g}\t{sm}\t{case}\t{m}\t{n}\t{}\t{}\t{}\t{}\t{}",
                            p[j],
                            f[j],
                            e[j],
                            ratio(f[j]),
                            ratio(e[j])
                        );
                    }
                }
            }
        }
    }
}
