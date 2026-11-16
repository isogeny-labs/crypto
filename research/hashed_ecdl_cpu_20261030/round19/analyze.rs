// Prospective descriptive analysis: neutral arm labels; no timing significance
// or speedup verdict is permitted for this shared-host diagnostic panel.
use std::{collections::BTreeMap, env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let raw = fs::read_to_string(&args[1]).unwrap();
    let mut cells = BTreeMap::<(String, usize), Vec<Vec<String>>>::new();
    let mut witnesses = BTreeMap::new();
    let mut seen = std::collections::BTreeSet::new();
    for line in raw.lines().skip(1) {
        let v: Vec<String> = line.split(',').map(str::to_owned).collect();
        assert_eq!(v.len(), 24);
        assert_eq!(v[5], "verified");
        let instance: usize = v[1].parse().unwrap();
        let round: usize = v[3].parse().unwrap();
        assert!(instance < 3 && round < 10);
        assert_eq!(v[2], (111 * (instance + 1)).to_string(), "unexpected seed");
        assert_eq!(
            v[6],
            ["42", "45678", "54321"][instance],
            "unexpected scalar"
        );
        assert_eq!(v[23], "L0", "unexpected isolation label");
        assert!(["u", "v", "w", "x", "y", "z"].contains(&v[0].as_str()));
        assert!(
            seen.insert((v[0].clone(), instance, round)),
            "duplicate cell"
        );
        assert_eq!(v[4], if round == 0 { "true" } else { "false" });
        let cert = fs::read_to_string(format!(
            "{}/cert-{}-{}-{}.txt",
            args[2], v[0], instance, round
        ))
        .unwrap();
        assert!(!cert.trim().is_empty(), "empty certificate");
        // Warmups remain part of the execution record and must be valid too.
        for column in (9..15).chain(std::iter::once(22)) {
            let value: f64 = v[column].parse().unwrap();
            assert!(value.is_finite() && value >= 0.0, "invalid duration");
        }
        for column in (7..9).chain(15..22) {
            let _: u64 = v[column].parse().unwrap();
        }
        let pair = (cert, v[6..9].join(","));
        if let Some(old) = witnesses.insert((instance, round), pair.clone()) {
            assert_eq!(old, pair, "witness or normalization output differs");
        }
        if v[4] == "false" {
            cells.entry((v[0].clone(), instance)).or_default().push(v);
        }
    }
    assert_eq!(seen.len(), 180);
    assert_eq!(cells.len(), 18);
    assert_eq!(witnesses.len(), 30);
    let mut csv = String::from("arm,instance,metric,n,min_ms,median_ms,max_ms\n");
    let mut bars = Vec::new();
    for ((arm, instance), rows) in &cells {
        assert_eq!(rows.len(), 9);
        for (column, name) in [
            (9, "curve"),
            (10, "bases"),
            (11, "oracle"),
            (12, "reduction_other"),
            (13, "replay"),
            (14, "complete"),
        ] {
            let mut values: Vec<f64> = rows.iter().map(|r| r[column].parse().unwrap()).collect();
            assert!(values.iter().all(|v| v.is_finite() && *v >= 0.0));
            values.sort_by(f64::total_cmp);
            csv.push_str(&format!(
                "{arm},{instance},{name},9,{},{},{}\n",
                values[0], values[4], values[8]
            ));
            if column == 14 {
                bars.push((
                    format!("{arm} / case {instance}"),
                    values[0],
                    values[4],
                    values[8],
                ));
            }
        }
    }
    let max = bars.iter().map(|b| b.3).fold(0.0_f64, f64::max).max(0.001);
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"900\" height=\"650\"><rect width=\"900\" height=\"650\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"16\" fill=\"#17212b\"><text x=\"25\" y=\"30\">CPU complete recovery (ms), median and full range, n=9</text><text x=\"25\" y=\"54\">Shared-host L0 diagnostics; no certified timing comparison</text>");
    for (i, (label, lo, med, hi)) in bars.iter().enumerate() {
        let y = 85 + i * 29;
        let scale = 550.0 / max;
        svg.push_str(&format!("<text x=\"25\" y=\"{}\">{label}</text><rect x=\"150\" y=\"{y}\" width=\"{}\" height=\"20\" fill=\"#3678a8\"/><line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\" stroke=\"#17212b\"/><text x=\"720\" y=\"{}\">{med:.3}</text>", y+16, med*scale, 150.0+lo*scale,150.0+hi*scale,y+10,y+10,y+16));
    }
    svg.push_str("<text x=\"25\" y=\"635\">HASHED-ECDL-CPU-20261030 / raw rows.csv; labels decoded in report</text></g></svg>");
    fs::write(format!("{}/summary.csv", args[2]), csv).unwrap();
    fs::write(format!("{}/latency.svg", args[2]), svg).unwrap();
    println!("PASS 180 rows, 162 measured, 18 complete cells, byte-identical full certificates");
}
