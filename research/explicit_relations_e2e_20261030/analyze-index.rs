// Frozen descriptive analysis. All rows, including censored runs, remain visible.
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};
fn main() {
    let a: Vec<String> = env::args().collect();
    let raw = fs::read_to_string(&a[1]).unwrap();
    let mut seen = BTreeSet::new();
    let mut rungs = BTreeMap::<u64, Vec<Vec<String>>>::new();
    for line in raw.lines().skip(1) {
        let v: Vec<String> = line.split(',').map(str::to_owned).collect();
        assert_eq!(v.len(), 19);
        let bits: u64 = v[1].parse().unwrap();
        let seed: u64 = v[2].parse().unwrap();
        assert!([101, 202].contains(&seed));
        assert!(["u", "x", "b"].contains(&v[0].as_str()));
        assert!(seen.insert((bits, seed, v[0].clone())));
        assert_eq!(v[4], "false");
        assert_eq!(v[18], "L0");
        assert!(["succeeded", "timeout", "limited"].contains(&v[5].as_str()));
        assert_eq!(v[7], if v[5] == "succeeded" { "1" } else { "0" });
        assert_eq!(
            v[6],
            if v[5] == "succeeded" {
                "verified"
            } else {
                "not_run"
            }
        );
        for col in [8, 9, 10] {
            let x: f64 = v[col].parse().unwrap();
            assert!(x.is_finite() && x >= 0.0);
        }
        rungs.entry(bits).or_default().push(v);
    }
    let mut expected = [48u64, 52, 56, 58, 60].into_iter();
    let mut stopped = false;
    let mut complete = 0;
    let mut success = 0;
    let mut count = 0;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"560\"><rect width=\"1000\" height=\"560\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"16\" fill=\"#182a3a\"><text x=\"30\" y=\"30\">RTX PRO 6000 complete recovery ladder: every execution</text><text x=\"30\" y=\"56\">L0 diagnostics; solver seconds include input generation, process startup and teardown</text>");
    for (bits, rows) in &rungs {
        assert!(!stopped);
        assert_eq!(Some(*bits), expected.next());
        assert_eq!(rows.len(), if *bits <= 52 { 6 } else { 4 });
        let ok = rows.iter().all(|v| v[5] == "succeeded");
        if ok {
            complete = *bits;
        } else {
            stopped = true;
        }
        for v in rows {
            count += 1;
            success += usize::from(v[5] == "succeeded");
            let seconds: f64 = v[9].parse().unwrap();
            let x = 100.0 + (*bits as f64 - 48.0) * 70.0;
            let y = 440.0 - 80.0 * (seconds.max(0.01).log10() + 2.0);
            let color = if v[5] != "succeeded" {
                "#cc3d3d"
            } else if v[0] == "b" {
                "#629e51"
            } else {
                "#2879b8"
            };
            let offset = if v[0] == "x" {
                5.0
            } else if v[0] == "b" {
                -5.0
            } else {
                0.0
            };
            svg.push_str(&format!("<circle cx=\"{}\" cy=\"{y}\" r=\"4\" fill=\"{color}\"><title>{bits} bits / {} / seed {} / {} / {seconds:.3}s</title></circle>",x+offset,v[0],v[2],v[5]));
        }
    }
    for e in -2..=3 {
        let y = 440.0 - 80.0 * (e as f64 + 2.0);
        svg.push_str(&format!("<line x1=\"80\" x2=\"930\" y1=\"{y}\" y2=\"{y}\" stroke=\"#ccd4db\"/><text x=\"18\" y=\"{}\">{}s</text>",y+5.0,10f64.powi(e)));
    }
    for bits in [48, 52, 56, 58, 60] {
        let x = 100 + (bits - 48) * 70;
        svg.push_str(&format!("<text x=\"{}\" y=\"470\">{bits}</text>", x - 10));
    }
    svg.push_str("<text x=\"400\" y=\"500\">Prime-field bits</text><text x=\"30\" y=\"530\">Blue: GPU u/x; green: binary-search baseline; red: retained timeout/limit. Two seeds per arm.</text></g></svg>");
    fs::write(format!("{}/scaling.svg", a[2]), svg).unwrap();
    fs::write(format!("{}/summary.txt",a[2]),format!("executions={count}\nverified={success}\nlargest_complete_bits={complete}\nstopped_at_bound={stopped}\n")).unwrap();
    println!("PASS {count} unique rows; {success} independently replayed recoveries; largest complete rung {complete} bits; censored={stopped}");
}
