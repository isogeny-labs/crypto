// Frozen descriptive analysis: retain every cell, verify exact matched work,
// and report paired ratios plus repeat spread without population inference.
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
};
fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn main() {
    let a: Vec<String> = env::args().collect();
    let out = Path::new(&a[1]);
    let raw = fs::read_to_string(out.join("rows.csv")).unwrap();
    let mut seen = BTreeSet::new();
    let mut groups = BTreeMap::<(u64, u64, usize), BTreeMap<String, Vec<String>>>::new();
    let mut verified = 0;
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1050\" height=\"600\"><rect width=\"1050\" height=\"600\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"15\" fill=\"#182a3a\"><text x=\"30\" y=\"30\">Uninstrumented full-cache specialization: complete recoveries</text><text x=\"30\" y=\"55\">Every execution; log10 cold solver seconds; L0 diagnostic clocks</text>");
    for sec in [1.0_f64, 10.0, 100.0, 1000.0, 10000.0] {
        let y = 500.0 - 100.0 * sec.log10();
        svg += &format!(
            "<path d=\"M80 {y}H1010\" stroke=\"#ddd\"/><text x=\"10\" y=\"{}\">{sec}s</text>",
            y + 5.0
        );
    }
    for line in raw.lines().skip(1) {
        let v: Vec<String> = line.split(',').map(str::to_owned).collect();
        assert_eq!(v.len(), 19);
        let bits: u64 = v[1].parse().unwrap();
        let seed: u64 = v[2].parse().unwrap();
        let rep: usize = v[3].parse().unwrap();
        assert!([52, 63].contains(&bits) && [101, 202].contains(&seed));
        assert!(rep < if bits == 52 { 5 } else { 1 });
        assert!(["b", "v", "x"].contains(&v[0].as_str()));
        assert!(seen.insert((bits, seed, rep, v[0].clone())));
        assert_eq!(v[4], "false");
        assert_eq!(v[18], "L0");
        assert!(["succeeded", "timeout", "limited", "failed"].contains(&v[5].as_str()));
        let ok = v[5] == "succeeded";
        assert_eq!(v[6], if ok { "verified" } else { "not_run" });
        assert_eq!(v[7], if ok { "1" } else { "0" });
        for col in [8, 9, 10] {
            let n: f64 = v[col].parse().unwrap();
            assert!(n.is_finite() && n >= 0.0);
        }
        let id = format!("{bits}-{seed}-{rep}-{}", v[0]);
        let exit = fs::read_to_string(out.join(format!("{id}.exit"))).unwrap();
        if ok {
            assert_eq!(exit.trim(), "0");
            verified += 1;
            let cert = fs::read(out.join(format!("{id}.cert"))).unwrap();
            assert!(!cert.is_empty());
            assert!(fs::read_to_string(out.join(format!("{id}.verify")))
                .unwrap()
                .contains("VERIFIED scalar="));
            assert!(!fs::read(out.join(format!("{id}.negative")))
                .unwrap()
                .is_empty());
        }
        let x = 180.0
            + if bits == 63 { 440.0 } else { 0.0 }
            + if seed == 202 { 220.0 } else { 0.0 }
            + rep as f64 * 6.0
            - 12.0;
        let secs: f64 = v[9].parse().unwrap();
        let y = 500.0 - 100.0 * secs.max(0.01).log10();
        let color = match v[0].as_str() {
            "b" => "#225ea8",
            "v" => "#238b45",
            _ => "#e6550d",
        };
        svg+=&format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"5\" fill=\"{color}\"><title>{id}: {secs}s {}</title></circle>",v[5]);
        groups
            .entry((bits, seed, rep))
            .or_default()
            .insert(v[0].clone(), v);
    }
    let mut ratios=String::from("bits,seed,replicate,candidate_over_baseline_mean,aa_relative_spread,b_seconds,v_seconds,x_seconds,oracle_ratio\n");
    let mut aggregated = BTreeMap::<(u64, u64), [Vec<f64>; 5]>::new();
    let mut largest = 0;
    let mut stopped = false;
    for bits in [52, 63] {
        let n = if bits == 52 { 5 } else { 1 };
        let expected = n * 2;
        let found = groups.keys().filter(|(b, _, _)| *b == bits).count();
        if stopped {
            assert_eq!(found, 0);
            continue;
        }
        assert_eq!(found, expected);
        let mut complete = true;
        for seed in [101, 202] {
            for rep in 0..n {
                let g = &groups[&(bits, seed, rep)];
                assert_eq!(g.len(), 3);
                if !g.values().all(|r| r[5] == "succeeded") {
                    complete = false;
                    continue;
                }
                let b = &g["b"];
                let v = &g["v"];
                let x = &g["x"];
                for c in [11, 12] {
                    assert_eq!(b[c], v[c]);
                    assert_eq!(b[c], x[c]);
                }
                let canonical = fs::read(out.join(format!("{bits}-{seed}-0-b.cert"))).unwrap();
                for arm in ["b", "v", "x"] {
                    assert_eq!(
                        canonical,
                        fs::read(out.join(format!("{bits}-{seed}-{rep}-{arm}.cert"))).unwrap()
                    );
                    assert_eq!(
                        fs::read(out.join(format!("{bits}-{seed}-0-b.input"))).unwrap(),
                        fs::read(out.join(format!("{bits}-{seed}-{rep}-{arm}.input"))).unwrap()
                    );
                }
                let bs: f64 = b[9].parse().unwrap();
                let vs: f64 = v[9].parse().unwrap();
                let xs: f64 = x[9].parse().unwrap();
                let mean = (bs + xs) / 2.0;
                let ratio = vs / mean;
                let aa = (bs - xs).abs() / mean;
                let oracle = v[15].parse::<f64>().unwrap()
                    / ((b[15].parse::<f64>().unwrap() + x[15].parse::<f64>().unwrap()) / 2.0);
                assert!(mean > 0.0 && ratio.is_finite() && oracle.is_finite());
                ratios += &format!("{bits},{seed},{rep},{ratio},{aa},{bs},{vs},{xs},{oracle}\n");
                let group = aggregated.entry((bits, seed)).or_default();
                for (list, value) in group.iter_mut().zip([bs, vs, xs, ratio, aa]) {
                    list.push(value);
                }
            }
        }
        if complete {
            largest = bits;
        } else {
            stopped = true;
        }
    }
    let mut stats=String::from("bits,seed,rounds,b_median,b_min,v_median,v_min,x_median,x_min,paired_ratio_median,aa_max\n");
    for ((bits, seed), mut g) in aggregated {
        let n = g[0].len();
        let mut med = [0.0; 5];
        let mut min = [0.0; 5];
        for i in 0..5 {
            med[i] = median(&mut g[i]);
            min[i] = g[i][0];
        }
        stats += &format!(
            "{bits},{seed},{n},{},{},{},{},{},{},{},{}\n",
            med[0],
            min[0],
            med[1],
            min[1],
            med[2],
            min[2],
            med[3],
            g[4][n - 1]
        );
    }
    for (i, label) in [
        "52-bit seed101 (5 rounds)",
        "52-bit seed202 (5 rounds)",
        "63-bit seed101 (1 round)",
        "63-bit seed202 (1 round)",
    ]
    .iter()
    .enumerate()
    {
        svg += &format!(
            "<text x=\"{}\" y=\"540\" font-size=\"13\">{label}</text>",
            85 + i * 220
        );
    }
    svg+="<text x=\"80\" y=\"578\">Blue: baseline; green: full-cache candidate; orange: identical baseline repeat. No profiler attached.</text></g></svg>";
    fs::write(out.join("ratios.csv"), ratios).unwrap();
    fs::write(out.join("statistics.csv"), stats).unwrap();
    fs::write(out.join("scaling.svg"), svg).unwrap();
    fs::write(out.join("summary.txt"),format!("planned=36\nexecutions={}\nverified={verified}\nlargest_complete_bits={largest}\nstopped_at_bound={stopped}\n",seen.len())).unwrap();
    println!("PASS {} recorded cells, {verified} verified recoveries; descriptive paired ratios, medians/minima and A/A spreads",seen.len());
}
