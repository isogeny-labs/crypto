use std::{collections::BTreeMap, fs};
fn main() {
    let root = "research/hec_residual_span_20261116/c4-phases";
    let mut a = BTreeMap::new();
    for l in fs::read_to_string(format!("{root}/PHASES.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        let key = (
            v[0].parse::<u64>().unwrap(),
            v[1].parse::<u64>().unwrap(),
            v[2].parse::<u64>().unwrap(),
            v[3].parse::<u64>().unwrap(),
            v[4].parse::<u64>().unwrap(),
            v[5].to_string(),
            v[6].parse::<u64>().unwrap(),
        );
        let cs: Vec<u64> = v[7].split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(cs.len(), 16);
        assert!(a.insert(key, cs).is_none());
    }
    let mut totals = BTreeMap::new();
    for ((p, sm, c, rep, arm, s, m), v) in &a {
        let w = totals
            .entry((*p, *sm, *c, *rep, *arm, *m))
            .or_insert(vec![0u64; 16]);
        for j in 0..16 {
            w[j] += v[j]
        }
        if *rep == 0 {
            assert_eq!(*v, a[&(*p, *sm, *c, 1, *arm, s.clone(), *m)])
        }
    }
    for l in fs::read_to_string(format!("{root}/CONTROL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        let n = |i: usize| v[i].parse::<u64>().unwrap();
        let key = (n(0), n(2), n(3), n(4), n(5), n(6));
        let cs: Vec<u64> = v[14].split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(totals.remove(&key).unwrap(), cs)
    }
    assert!(totals.is_empty());
    println!("p\tsm\tcase\tphase\tREDC\ttotal_REDC\tshare\tdelete_phase_conditional_ceiling");
    for ((p, sm, c, rep, arm, s, m), v) in &a {
        if *rep != 0 || *arm != 0 || p != m {
            continue;
        }
        let total: u64 = a
            .iter()
            .filter(|((pp, ss, cc, rr, aa, _, mm), _)| {
                pp == p && ss == sm && cc == c && *rr == 0 && *aa == 0 && mm == m
            })
            .map(|(_, v)| v[8])
            .sum();
        println!(
            "{p}\t{sm}\t{c}\t{s}\t{}\t{total}\t{:.6}\t{:.6}",
            v[8],
            v[8] as f64 / total as f64,
            total as f64 / (total - v[8]) as f64
        )
    }
    eprintln!(
        "all_phase_sums_exact=true all_phase_repeats_exact=true records={}",
        a.len()
    );
}
