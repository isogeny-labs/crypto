use std::{collections::BTreeMap, fs};
fn main() {
    let root = "research/hec_residual_span_20261116";
    let mut old = BTreeMap::new();
    for (study, arm, g4) in [("orbit-core", 3u64, false), ("c4-integrated", 0, true)] {
        for l in fs::read_to_string(format!("{root}/{study}/CONTROL.tsv"))
            .unwrap()
            .lines()
            .skip(1)
        {
            let v: Vec<_> = l.split('\t').collect();
            let n = |j: usize| v[j].parse::<u64>().unwrap();
            if n(5) != arm {
                continue;
            }
            let key = if g4 {
                (4, n(0), n(2), n(3), n(4), n(6))
            } else {
                (n(0), n(1), n(2), n(3), n(4), n(6))
            };
            let cs: Vec<u64> = v[14].split(',').map(|x| x.parse().unwrap()).collect();
            let fields = if g4 {
                (
                    v[7].to_string(),
                    v[8].to_string(),
                    n(10),
                    Some((n(12), n(13))),
                )
            } else {
                (v[7].to_string(), v[8].to_string(), n(11), None)
            };
            assert!(old.insert(key, (fields, cs)).is_none());
        }
    }
    assert_eq!(old.len(), 128);
    let mut new = BTreeMap::new();
    println!("g\tp\tsm\tcase\tmodulus\told_over_new_REDC\told_over_new_inv\told_REDC\tnew_REDC\trows\tanchors\tt");
    for l in fs::read_to_string(format!("{root}/incremental-pencil/CONTROL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        assert_eq!(v.len(), 14);
        let n = |j: usize| v[j].parse::<u64>().unwrap();
        let key = (n(0), n(1), n(2), n(3), n(4), n(5));
        let (fields, o) = &old[&key];
        assert_eq!(fields.0, v[6]);
        assert_eq!(fields.1, v[7]);
        assert_eq!(v[8], "true");
        assert_eq!(v[6], v[7]);
        assert_eq!(fields.2, n(9));
        if let Some((a, t)) = fields.3 {
            assert_eq!((a, t), (n(10), n(11)))
        }
        let cs: Vec<u64> = v[13].split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(cs.len(), 16);
        assert_eq!(cs[8], 2 * cs[7] + cs[5] + cs[6]);
        if key.1 == key.5 {
            assert_eq!(o, &cs)
        }
        if key.4 == 1 {
            assert_eq!(new[&(key.0, key.1, key.2, key.3, 0, key.5)], cs)
        }
        assert!(new.insert(key, cs.clone()).is_none());
        if key.4 == 0 {
            println!(
                "{}\t{}\t{}\t{}\t{}\t{:.6}\t{:.6}\t{}\t{}\t{}\t{}\t{}",
                key.0,
                key.1,
                key.2,
                key.3,
                key.5,
                o[8] as f64 / cs[8] as f64,
                o[9] as f64 / cs[9] as f64,
                o[8],
                cs[8],
                n(9),
                n(10),
                n(11)
            )
        }
    }
    assert_eq!(new.len(), 128);
    eprintln!("records=128 scalar_prefixes_equal=true all_base_field_vectors_identical=true all_repeats=true REDC_identity=true");
}
