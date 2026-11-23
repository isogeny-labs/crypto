use std::{collections::BTreeMap, fs};
fn main() {
    let raw = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut last = BTreeMap::new();
    let mut first = BTreeMap::new();
    let mut counts = BTreeMap::new();
    for l in raw.lines().skip(1) {
        let v: Vec<_> = l.split('\t').map(str::to_string).collect();
        assert_eq!(v.len(), 15);
        let key = (
            v[0].parse::<usize>().unwrap(),
            v[2].parse::<u64>().unwrap(),
            v[3].parse::<usize>().unwrap(),
            v[4].parse::<usize>().unwrap(),
        );
        if v[14] != "none" {
            first
                .entry(key)
                .or_insert((v[5].clone(), v[6].clone(), v[7].clone()));
        }
        let gr = v[10].parse::<usize>().unwrap();
        let naive = v[11].parse::<usize>().unwrap();
        assert!(gr >= naive);
        *counts.entry((key.0, key.3)).or_insert((0usize, 0usize)) = (
            counts.get(&(key.0, key.3)).unwrap_or(&(0, 0)).0 + 1,
            counts.get(&(key.0, key.3)).unwrap_or(&(0, 0)).1 + usize::from(gr > naive),
        );
        last.insert(key, v);
    }
    assert_eq!(last.len(), 36);
    println!("g\tsm\tcase\tsmall_orbits\tpartials\trejected_to_last_accepted\tlp_rank\tleft_nullity\tlabel_rank\tfirst_determining_anchor\tfirst_t\tfirst_partials\tk");
    let mut groups = BTreeMap::new();
    for (key, v) in last {
        let a = first
            .get(&key)
            .cloned()
            .unwrap_or(("none".into(), "none".into(), "none".into()));
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            key.0, key.1, key.2, key.3, v[7], v[8], v[9], v[12], v[13], a.0, a.1, a.2, v[14]
        );
        let g = groups
            .entry((key.0, key.3))
            .or_insert((0usize, 0usize, usize::MAX, 0usize));
        g.0 += 1;
        g.1 += usize::from(v[14] != "none");
        let n = v[7].parse::<usize>().unwrap();
        g.2 = g.2.min(n);
        g.3 = g.3.max(n);
    }
    for ((g, cut), (cases, solved, lo, hi)) in groups {
        let (checks, bad) = counts[&(g, cut)];
        eprintln!("g={g} cut={cut} cases={cases} filtered_completed={solved} accepted_partial_range={lo}..{hi} graph_checks={checks} naive_overcount_prefixes={bad}");
    }
}
