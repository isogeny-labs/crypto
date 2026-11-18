use std::{collections::BTreeMap, fs};
fn main() {
    let root = "research/hec_residual_span_20261116";
    let read = |s: &str| -> Vec<Vec<String>> {
        fs::read_to_string(format!("{root}/{s}/CONTROL.tsv"))
            .unwrap()
            .lines()
            .skip(1)
            .map(|l| l.split('\t').map(str::to_string).collect())
            .collect()
    };
    let old = read("c4-solves");
    let new = read("c4-integrated");
    assert_eq!(old.len(), 144);
    assert_eq!(new.len(), 144);
    let mut all = BTreeMap::new();
    let mut changes = BTreeMap::new();
    for (o, n) in old.iter().zip(&new) {
        assert_eq!(&o[..14], &n[..14]);
        let get = |i: usize| n[i].parse::<u64>().unwrap();
        let oc: Vec<i128> = o[14].split(',').map(|x| x.parse().unwrap()).collect();
        let nc: Vec<i128> = n[14].split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(nc.len(), 16);
        assert_eq!(nc[8], 2 * nc[7] + nc[5] + nc[6]);
        let delta: Vec<_> = oc.iter().zip(&nc).map(|(a, b)| a - b).collect();
        let key = (get(0), get(2), get(6));
        if let Some(prev) = changes.insert(key, delta.clone()) {
            assert_eq!(prev, delta)
        }
        assert!(all
            .insert((get(0), get(2), get(3), get(4), get(5), get(6)), (oc, nc))
            .is_none());
    }
    println!("p\tsm\tcase\tmodulus\told_pencil_over_new_pencil_REDC\tnew_folded_over_new_pencil_REDC\told_folded_over_new_pencil_REDC\told_pencil_REDC\tnew_pencil_REDC");
    for (&(p, sm, c, rep, arm, m), (o, n)) in &all {
        if rep != 0 || arm != 0 {
            continue;
        }
        let (ow, nw) = &all[&(p, sm, c, 0, 1, m)];
        println!(
            "{p}\t{sm}\t{c}\t{m}\t{:.6}\t{:.6}\t{:.6}\t{}\t{}",
            o[8] as f64 / n[8] as f64,
            nw[8] as f64 / n[8] as f64,
            ow[8] as f64 / n[8] as f64,
            o[8],
            n[8]
        );
    }
    eprintln!("all_144_outcome_prefixes_identical=true setup_vector_delta_constant_by_model_source_modulus=true REDC_identity=true");
}
