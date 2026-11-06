use std::{collections::BTreeSet, env, fs};
fn main() {
    let a: Vec<_> = env::args().collect();
    let raw = fs::read_to_string(&a[1]).unwrap();
    let out = &a[2];
    let mut seen = BTreeSet::new();
    let mut bars = Vec::new();
    let mut certs = std::collections::BTreeMap::new();
    for line in raw.lines().skip(1) {
        let v: Vec<_> = line.split(',').collect();
        assert_eq!(v.len(), 24);
        assert_eq!(v[5], "verified");
        assert_eq!(v[23], "L0");
        let c: usize = v[1].parse().unwrap();
        let s: u64 = v[2].parse().unwrap();
        assert!(c <= 5 && [101, 202].contains(&s) && seen.insert((v[0].to_owned(), c, s)));
        assert_eq!(v[6].parse::<usize>().unwrap(), 20 + 4 * c);
        let verify = fs::read_to_string(format!("{out}/{}-{c}-{s}.verify", v[0])).unwrap();
        assert_eq!(verify.trim(), format!("VERIFIED scalar={}", v[8]));
        let t: f64 = v[11].parse().unwrap();
        assert!(t.is_finite() && t > 0.0);
        let cert = fs::read(format!("{out}/scale-{}-{c}-{s}.json", v[0])).unwrap();
        if let Some(old) = certs.insert((c, s), cert.clone()) {
            assert_eq!(old, cert);
        }
        if v[0] == "u" {
            bars.push((format!("{} bits / {s}", 20 + 4 * c), t));
        }
    }
    let status = fs::read_to_string(format!("{out}/status.csv")).unwrap();
    let mut attempts = BTreeSet::new();
    let mut failures = 0;
    for line in status.lines().skip(1) {
        let v: Vec<_> = line.split(',').collect();
        assert_eq!(v.len(), 5);
        let key = (
            v[0].to_owned(),
            v[1].parse::<usize>().unwrap(),
            v[2].parse::<u64>().unwrap(),
        );
        assert!(["u", "x"].contains(&v[0]));
        assert!(attempts.insert(key.clone()));
        if v[3] == "0" && v[4] == "0" {
            assert!(seen.contains(&key));
        } else {
            failures += 1;
        }
    }
    assert!(!attempts.is_empty() && attempts.len() <= 24);
    assert_eq!(attempts.len() % 4, 0);
    let levels = attempts.len() / 4;
    for c in 0..levels {
        for seed in [101, 202] {
            for arm in ["u", "x"] {
                let key = (arm.to_owned(), c, seed);
                assert!(attempts.contains(&key));
                if c + 1 < levels {
                    assert!(seen.contains(&key));
                }
            }
        }
    }
    assert!(levels == 6 || failures > 0);
    println!(
        "planned=24 attempted={} verified={} failed={} not_attempted={}",
        attempts.len(),
        seen.len(),
        failures,
        24 - attempts.len()
    );
    let max = bars.iter().map(|b| b.1).fold(0.0_f64, f64::max).max(0.001);
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"900\" height=\"540\"><rect width=\"900\" height=\"540\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"15\" fill=\"#17212b\"><text x=\"20\" y=\"28\">Complete CPU public-target recovery, seconds (L0 diagnostics)</text><text x=\"20\" y=\"52\">Two public targets per level; only independently replayed completions shown</text>");
    for (i, (label, t)) in bars.iter().enumerate() {
        let y = 80 + i * 34;
        svg.push_str(&format!("<text x=\"20\" y=\"{}\">{label}</text><rect x=\"180\" y=\"{y}\" width=\"{}\" height=\"19\" fill=\"#3678a8\"/><text x=\"750\" y=\"{}\">{t:.3}</text>",y+15,520.0*t/max,y+15));
    }
    svg.push_str("</g></svg>");
    fs::write(format!("{out}/scale.svg"), svg).unwrap();
}
