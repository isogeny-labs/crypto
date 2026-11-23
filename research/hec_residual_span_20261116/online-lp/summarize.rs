use std::{collections::BTreeMap, fs};
type Key = (usize, u64, usize, usize);
fn key(v: &[String]) -> Key {
    (
        v[0].parse().unwrap(),
        v[2].parse().unwrap(),
        v[3].parse().unwrap(),
        v[4].parse().unwrap(),
    )
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let raw = fs::read_to_string(&args[1]).unwrap();
    let full = fs::read_to_string(&args[2]).unwrap();
    let mut f = BTreeMap::new();
    for l in full.lines().skip(1) {
        let v: Vec<_> = l.split('\t').map(str::to_string).collect();
        assert_eq!(v.len(), 14);
        f.insert((key(&v), v[5].clone(), v[6].clone()), v);
    }
    let mut last = BTreeMap::new();
    let mut first = BTreeMap::new();
    for l in raw.lines().skip(1) {
        let v: Vec<_> = l.split('\t').map(str::to_string).collect();
        assert_eq!(v.len(), 20);
        let k = key(&v);
        if v[19] != "none" {
            first.entry(k).or_insert(v.clone());
        }
        last.insert(k, v);
    }
    assert_eq!(last.len(), 36);
    println!("g\tsm\tcase\tcut\tstatus\tanchor\tt\tpartials\tonline_cells\tinput_cells\tfull_incremental_cells\tonline_redc\tscratch_redc\tfull_incremental_redc\tscratch_over_online_redc\tfull_incremental_over_online_redc\tk");
    let mut grouped: BTreeMap<(usize, usize, String), Vec<(f64, f64)>> = BTreeMap::new();
    for (k, v) in last {
        let v = first.get(&k).unwrap_or(&v);
        let z = &f[&(k, v[5].clone(), v[6].clone())];
        assert_eq!(z[13], v[19]);
        let n = v[15].parse::<f64>().unwrap();
        assert!(n > 0.0);
        let scratch = v[18].parse::<f64>().unwrap() / n;
        let incremental = z[12].parse::<f64>().unwrap() / n;
        let status = if v[19] == "none" {
            "incomplete"
        } else {
            "complete"
        };
        println!("{}\t{}\t{}\t{}\t{status}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{scratch:.6}\t{incremental:.6}\t{}",k.0,k.1,k.2,k.3,v[5],v[6],v[7],v[10],v[11],z[9],v[15],v[18],z[12],v[19]);
        grouped
            .entry((k.0, k.3, status.into()))
            .or_default()
            .push((scratch, incremental));
    }
    for ((g, cut, status), v) in grouped {
        let mm = |j: usize| {
            let z: Vec<_> = v.iter().map(|x| if j == 0 { x.0 } else { x.1 }).collect();
            (
                z.iter().copied().fold(f64::INFINITY, f64::min),
                z.iter().copied().fold(0.0, f64::max),
            )
        };
        eprintln!("g={g} cut={cut} status={status} streams={} scratch_over_online={:?} full_incremental_over_online={:?}",v.len(),mm(0),mm(1));
    }
}
