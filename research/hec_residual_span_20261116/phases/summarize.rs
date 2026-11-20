use std::{collections::BTreeMap, fs};
fn main() {
    let raw = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut cells: BTreeMap<(usize, u64, usize, String, String), u64> = BTreeMap::new();
    for l in raw.lines().skip(1) {
        let v: Vec<_> = l.split('\t').collect();
        assert_eq!(v.len(), 24);
        if v[5] != "0" || v[1] != v[7] {
            continue;
        }
        cells.insert(
            (
                v[0].parse().unwrap(),
                v[2].parse().unwrap(),
                v[3].parse().unwrap(),
                v[4].into(),
                v[6].into(),
            ),
            v[16].parse().unwrap(),
        );
    }
    println!("g\tsm\tcase\tarm\tbase_redc\tpreparation_redc\tpreparation_fraction\tunchanged_preparation_redc_ceiling");
    for g in 2..=4 {
        let mut fractions = vec![];
        let mut ceilings = vec![];
        for sm in [1u64, 17] {
            for case in 0..3 {
                for arm in ["pencil", "folded", "expanded"] {
                    let mut total = 0;
                    let mut prep = 0;
                    for ((gg, ss, cc, aa, phase), &n) in &cells {
                        if *gg == g && *ss == sm && *cc == case && aa == arm {
                            total += n;
                            if ["entry", "factor_base", "action"].contains(&phase.as_str()) {
                                prep += n
                            }
                        }
                    }
                    assert!(prep > 0);
                    let frac = prep as f64 / total as f64;
                    let ceiling = total as f64 / prep as f64;
                    println!("{g}\t{sm}\t{case}\t{arm}\t{total}\t{prep}\t{frac:.6}\t{ceiling:.6}");
                    if arm == "pencil" {
                        fractions.push(frac)
                    }
                    if arm == "folded" {
                        ceilings.push(ceiling)
                    }
                }
            }
        }
        let minmax = |x: &[f64]| {
            (
                x.iter().copied().fold(f64::INFINITY, f64::min),
                x.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            )
        };
        eprintln!("g={g} pencil_preparation_fraction={:?} folded_redc_ceiling_if_preparation_unchanged={:?}",minmax(&fractions),minmax(&ceilings));
    }
}
