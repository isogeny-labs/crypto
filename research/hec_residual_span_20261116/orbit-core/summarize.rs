use std::{collections::BTreeMap, fs};
fn main() {
    let root = "research/hec_residual_span_20261116/orbit-core";
    let mut all = BTreeMap::new();
    for l in fs::read_to_string(format!("{root}/CONTROL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        assert_eq!(v.len(), 15);
        assert_eq!(v[9], "true");
        assert_eq!(v[7], v[8]);
        let n = |i: usize| v[i].parse::<u64>().unwrap();
        let cs: Vec<u64> = v[14].split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(cs.len(), 16);
        assert_eq!(cs[8], 2 * cs[7] + cs[5] + cs[6]);
        assert!(all
            .insert(
                (n(0), n(1), n(2), n(3), n(4), n(5), n(6)),
                (cs, n(10), n(11), n(12), n(13))
            )
            .is_none());
    }
    assert_eq!(all.len(), 320);
    println!("g\tp\tsm\tcase\tmodulus\told_folded_over_core_sparse_REDC\tbatch_over_core_sparse_REDC\tsparse_over_online_REDC\tcore_sparse_REDC\tcore_trials\told_trials\taccepted\trejected\tsmall");
    for (&(g, p, sm, c, rep, arm, m), row) in &all {
        if rep != 0 || arm != 1 {
            continue;
        }
        for a in 0..4 {
            assert_eq!(all[&(g, p, sm, c, 0, a, m)], all[&(g, p, sm, c, 1, a, m)])
        }
        let online = &all[&(g, p, sm, c, 0, 0, m)];
        assert_eq!(
            (row.1, row.2, row.3, row.4),
            (online.1, online.2, online.3, online.4)
        );
        let old = &all[&(g, p, sm, c, 0, 2, m)];
        let batch = &all[&(g, p, sm, c, 0, 3, m)];
        println!(
            "{g}\t{p}\t{sm}\t{c}\t{m}\t{:.6}\t{:.6}\t{:.6}\t{}\t{}\t{}\t{}\t{}\t{}",
            old.0[8] as f64 / row.0[8] as f64,
            batch.0[8] as f64 / row.0[8] as f64,
            row.0[8] as f64 / online.0[8] as f64,
            row.0[8],
            row.1,
            old.1,
            row.2,
            row.3,
            row.4
        )
    }
    eprintln!("raw_modulus_records=320 all_scalars_match=true all_repeats=true REDC_identity=true fresh_workloads=20");
}
