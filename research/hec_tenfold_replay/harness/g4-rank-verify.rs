use hec_native_ic::instance::thread_curve;
use std::{collections::BTreeMap, env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut groups = BTreeMap::new();
    for line in fs::read_to_string(&args[1]).unwrap().lines() {
        let r: Vec<_> = line.split_whitespace().collect();
        if r[2] == "selected" {
            let parse = |i: usize| r[i].parse::<u64>().unwrap();
            assert!(groups
                .insert((parse(0), parse(1)), (parse(3), parse(4)))
                .is_none());
        }
    }
    let mut count = 0;
    for line in fs::read_to_string(&args[2]).unwrap().lines() {
        let r: Vec<u64> = line
            .split_whitespace()
            .skip(1)
            .map(|t| t.parse().unwrap())
            .collect();
        let (p, c, l, anchor) = (r[0], r[1], r[2], r[3]);
        let (order, prime) = groups.remove(&(p, c)).unwrap();
        assert_eq!(prime, l);
        let curve = thread_curve(p, c, 4).unwrap();
        let places: Vec<_> = (0..p)
            .filter_map(|x| {
                let y = curve.fp.sqrt(curve.f.eval(x, &curve.fp))?;
                (y != 0).then(|| (x, curve.place(x, y.min(p - y))))
            })
            .collect();
        assert_eq!(r.len(), 4 + places.len());
        let a = places.iter().position(|(x, _)| *x == anchor).unwrap();
        let project = |point| curve.mul(point, order / l);
        let base = project(&places[a].1);
        assert!(!curve.is_zero(&base) && curve.is_zero(&curve.mul(&base, l)));
        for (i, (_, place)) in places.iter().enumerate() {
            assert!(r[4 + i] < l);
            assert_eq!(
                curve.mul(&project(place), r[4 + a]),
                curve.mul(&base, r[4 + i]),
                "p={p} c={c} i={i}"
            );
            count += 1;
        }
    }
    println!(
        "verified {count} place coordinates; {} curves without a submitted vector",
        groups.len()
    );
}
